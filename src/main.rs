#![feature(mutex_unlock)]
extern crate redis;
use redis::Commands;
// use redis::cmd;

use std::process::{Command, Stdio};
// use threadpool::ThreadPool;
use tiny_keccak::Sha3;
use tiny_keccak::Hasher;
use rand::Rng;
use std::time::Duration;
use std::thread::sleep;
// use std::sync::{Arc, Mutex};
use std::thread;
// ntex
use std::{env, io};
// use log::info;
// use ntex::http::header::HeaderValue;
// use ntex::http::{HttpService, Response};
// use ntex::{server::Server, time::Seconds, util::Ready};
use ntex::time::Seconds;
use std::fmt::Write as FmtWrite;
use ntex::http;
use ntex::web::{self, middleware, App, HttpServer};


const MAX_RUN: i64 = 1000000000000000000;
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
const PASSWORD_LEN: usize = 10000000;
// const PASSWORD_LEN: usize = 10000;
const REDIS_URL: &str = "redis://:111111@127.0.0.1/";

// fn task(t: usize, c: Arc<Mutex<i32>>) {
fn task(t: usize) {

    // let mut num = c.lock().unwrap();
    // *num += 1;
    // println!("{}: {}", t, *c.lock().unwrap());
    println!("Start task {}", t);
    let client = redis::Client::open(REDIS_URL).unwrap();
    loop {
        // println!("start task");
        let mut con = client.get_connection().unwrap();
        let mut n: i64 = 0;
        while n < MAX_RUN {
            // let mut num = c.lock().unwrap();
            // *num += 1;
            n += 1;
            redis::cmd("INCR").arg("counter").query::<i32>(&mut con).unwrap();
            //con.incr(key, delta);
            // continue;
            // Mutex::unlock(num);
            // drop(num);
            // println!("sub: {}", n);
            let mut sha3 = Sha3::v256();
            let mut output = [0u8; 32];
            
            let mut rng = rand::thread_rng();
            
            let password: String = (0..PASSWORD_LEN)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
            .collect();
                
            sha3.update(b"hello");
            sha3.update(password.as_bytes());
            sha3.update(b"world");
            sha3.finalize(&mut output);
            // let l = std::str::from_utf8(&output).unwrap();
            // let mut l = String::new();
            let l = output.iter().map(|x| format!("{:02x}", x)).collect::<String>();
            // println!("{:#?}", &l);
            Command::new("echo")
                .arg(&l)
                .stdout(Stdio::null())
                .output()
                .expect("Failed to execute command");
            n += 1;
        }
    }
    
}

#[ntex::main]
async fn main() -> io::Result<()> {
    // env::set_var("RUST_LOG", "ntex=trace");
    env::set_var("RUST_LOG", "ntex=info");
    println!("System up");
    env_logger::init();

    // clear data
    let client = redis::Client::open(REDIS_URL).unwrap();
    let mut con = client.get_connection().unwrap();
    //let _rt = con.del("counter").unwrap();
    println!("clean redis");
    redis::cmd("DEL").arg("counter").query::<i32>(&mut con).unwrap();
    drop(con);
    drop(client);
    sleep(Duration::from_secs(1));

    let cpus = num_cpus::get();
    println!("cpu number: {}", cpus);
    // let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for c in "我是谁".chars() {
        println!("{}", c);
    }

    // let pool = ThreadPool::new(cpus);

    // let mut c = 0;

    // let counter_thread = thread::spawn(move || {
    //     let c = Arc::clone(&counter);
    // });
    // handles.push(counter_thread);
    
    for i in 0..cpus+1 {
        // c = c + 1;
        // println!("{}", i);
        // pool.execute(|| { 
        // let counter = Arc::clone(&counter);
        if i == 0 {
            let handle = thread::spawn(move || {
                // println!("before task");
                // *counter.lock().unwrap() += 1;
                sleep(Duration::from_secs(1));
                println!("Start counter");
                let mut last = 0;
                let client = redis::Client::open(REDIS_URL).unwrap();
                loop {
                    // let (tx, rx) = ntex::channel::oneshot::channel::<i32>();
                    // drop(rx);
                    let mut con = client.get_connection().unwrap();
                    // let tmp = *counter.lock().unwrap();
                    let tmp = con.get("counter").unwrap();
                    println!("counter: {}, D: {}", tmp, tmp-last);
                    //redis::cmd("INCR").arg("counter").arg(tmp).query::<i32>(&mut con).unwrap();
                    // tx.send(tmp).unwrap();
                    // tx.send(tmp).unwrap();
                    // drop(tx);
                    last = tmp;
                    sleep(Duration::from_secs(1));
                }
                // println!("after");
            });
            handles.push(handle);
        } else {
            let handle = thread::spawn(move || {
                // println!("before task");
                // *counter.lock().unwrap() += 1;
                //task(i, counter);
                task(i);
                // println!("after");
            });
            handles.push(handle);
        }
    }

    // for handle in handles {
    //    handle.join().unwrap();
    //}

    // sleep(Duration::from_secs(1));
    // println!("actived pool: {}", pool.active_count());
    // pool.join();
    // println!("Result: {}", *counter.lock().unwrap());
    

    println!("Start Ntex Server");

    // Server::build()
    //     .bind("hello-world", "127.0.0.1:3080", |_| {
    //         HttpService::build()
    //             .client_timeout(Seconds(1))
    //             .disconnect_timeout(Seconds(1))
    //             .finish(|_req| {
    //                 info!("{:?}", _req);
    //                 // let c = rx;
    //                 let mut rt_str = String::new();
    //                 write!(&mut rt_str, "Result: {}", "1").unwrap();
    //                 let mut res = Response::Ok();
    //                 res.header("x-head", HeaderValue::from_static("dummy value!"));
    //                 Ready::Ok::<_, io::Error>(res.body(rt_str))
    //             })
    //     })?
    //     .workers(1)
    //     .run()
    //     .await
    

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(|| async { 
                let client = redis::Client::open(REDIS_URL).unwrap();
                let mut con = client.get_connection().unwrap();
                let tmp: i32 = con.get("counter").unwrap();
                let mut s = String::new();
                write!(s, "counter: {}", tmp).unwrap();
                s
            }))
    })
    .bind("0.0.0.0:3080")?
    .client_timeout(Seconds(1))
    .workers(1)
    .keep_alive(http::KeepAlive::Disabled)
    .run()
    .await

}
