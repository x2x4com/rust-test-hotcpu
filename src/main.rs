// #![feature(mutex_unlock)]
// extern crate redis;
// use redis::Commands;
// use redis::cmd;

// use std::process::{Command, Stdio};
// use threadpool::ThreadPool;
use tiny_keccak::Sha3;
use tiny_keccak::Hasher;
use rand::Rng;
use std::time::Duration;
use std::thread::sleep;
use std::sync::{Arc, Mutex};
use std::thread;
// ntex
use std::{env, io};
// use log::info;
// use ntex::http::header::HeaderValue;
// use ntex::http::{HttpService, Response};
// use ntex::{server::Server, time::Seconds, util::Ready};
use ntex::time::Seconds;
// use std::fmt::Write as FmtWrite;
use ntex::http;
use ntex::web::{self, middleware, App, HttpServer, HttpResponse};
// use clap::{Parser, Subcommand, ColorChoice, value_parser, Args};
use clap::{Parser, ColorChoice};
// use clap::{Parser, Subcommand, ValueEnum};
use owo_colors::{OwoColorize, Stream};
// use std::cell::Cell;



const MAX_RUN: i64 = 1000000000000000000;
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
const PASSWORD_LEN: usize = 10000000;
// const PASSWORD_LEN: usize = 10000;
// const REDIS_URL: &str = "redis://:111111@127.0.0.1/";

#[derive(Debug, Parser)]
#[command(version="1.0", author="Jack Xu")]
// #[command(setting = AppSettings::ColoredHelp)]
struct Cli {
    #[arg(short,long,value_name="Max Run", default_value_t=MAX_RUN)]
    max_run: i64,
    #[arg(short,long,value_name="PASSWORD LEN", default_value_t=PASSWORD_LEN)]
    pass_len: usize,
    #[arg(short,long,value_name="Reader interval(ms)", default_value_t=1000u64)]
    interval: u64,
    #[arg(
        short,
        long,
        require_equals = true,
        value_name = "WHEN",
        num_args = 0..=2,
        default_value_t = ColorChoice::Auto,
        // default_missing_value = "auto",
        value_enum
    )]
    color: ColorChoice,
}

#[derive(Debug, Clone)]
struct MyState {
    counter: Arc<Mutex<i32>>,
}

fn task(t: usize, max_run: i64, pass_len: usize, locker: Arc<Mutex<i32>>) {
// fn task(t: usize) {

    
    // *num += 1;
    // println!("{}: {}", t, *c.lock().unwrap());
    println!("Start task {}", t);
    // let client = redis::Client::open(REDIS_URL).unwrap();
    loop {
        let mut n: i64 = 0;
        
        // let mut con = client.get_connection().unwrap();
        while n < max_run {
            // println!("Round {}-{}", t, n);
            // let mut num = c.lock().unwrap();
            // *num += 1;
            n += 1;
            // redis::cmd("INCR").arg("counter").query::<i32>(&mut con).unwrap();
            //con.incr(key, delta);
            // continue;
            // Mutex::unlock(num);
            // drop(num);
            // println!("sub: {}", n);
            let mut sha3 = Sha3::v256();
            let mut output = [0u8; 32];
            
            let mut rng = rand::thread_rng();
            
            let password: String = (0..pass_len)
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
            let _l = output.iter().map(|x| format!("{:02x}", x)).collect::<String>();
            // println!("{:#?}", &l);
            // Command::new("echo")
            //     .arg(&l)
            //     .stdout(Stdio::null())
            //     .output()
            //     .expect("Failed to execute command");
            n += 1;
            let mut num = locker.lock().unwrap();
            *num += 1;
        }
    }
    
}

async fn http_index(st: web::types::State<MyState>) -> HttpResponse {
// async fn http_index(st: web::types::State<MyState>) -> i32 {
    let c = st.counter.lock().unwrap();
    println!("{}", *c);
    let s = c.to_string();
    HttpResponse::Ok().body(s)
}

#[ntex::main]
async fn main() -> io::Result<()> {
    // env::set_var("RUST_LOG", "ntex=trace");
    env::set_var("RUST_LOG", "ntex=info");
    println!("System up");
    env_logger::init();

    let args = Cli::parse();
    // println!("{:?}", args);
    // init color
    match args.color {
        ColorChoice::Always => owo_colors::set_override(true),
        ColorChoice::Auto => {}
        ColorChoice::Never => owo_colors::set_override(false),
    }

    // clear data
    // let client = redis::Client::open(REDIS_URL).unwrap();
    // let mut con = client.get_connection().unwrap();
    //let _rt = con.del("counter").unwrap();
    // println!("clean redis");
    // redis::cmd("DEL").arg("counter").query::<i32>(&mut con).unwrap();
    // drop(con);
    // drop(client);
    sleep(Duration::from_secs(1));

    let cpus = num_cpus::get();
    println!("cpu number: {}", cpus.if_supports_color(Stream::Stdout, |text| text.red()));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // for c in "我是谁".chars() {
    //     println!("{}", c);
    // }

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
        let locker = Arc::clone(&counter);
        if i == 0 {
            let handle = thread::spawn(move || {
                // println!("before task");
                // *counter.lock().unwrap() += 1;
                println!("Start counter");
                sleep(Duration::from_secs(1));
                // let mut last = 0;
                // let client = redis::Client::open(REDIS_URL).unwrap();
                loop {
                    // let (tx, rx) = ntex::channel::oneshot::channel::<i32>();
                    // drop(rx);
                    // let mut con = client.get_connection().unwrap();
                    // let tmp = *counter.lock().unwrap();
                    // let tmp = con.get("counter").unwrap();
                    // println!("counter: {}, D: {}", tmp, tmp-last);
                    //redis::cmd("INCR").arg("counter").arg(tmp).query::<i32>(&mut con).unwrap();
                    // tx.send(tmp).unwrap();
                    // tx.send(tmp).unwrap();
                    // drop(tx);
                    // last = tmp;
                    let c = *locker.lock().unwrap();
                    // (8u8 as char) 会在原本基础上累加
                    print!("Counter: {}\r", c.if_supports_color(Stream::Stdout, |text| text.green()));
                    sleep(Duration::from_millis(args.interval));
                }
                // println!("after");
            });
            handles.push(handle);
        } else {
            let handle = thread::spawn(move || {
                // println!("before task");
                // *counter.lock().unwrap() += 1;
                //task(i, counter);
                task(i, args.max_run, args.pass_len, locker);
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
    

    HttpServer::new(move || {
        // let locker = Arc::clone(&counter);
        App::new()
            .state(MyState { counter: counter.clone() })
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/").route(web::get().to(http_index)))
    })
    .bind("0.0.0.0:3080")?
    .client_timeout(Seconds(1))
    .workers(1)
    .keep_alive(http::KeepAlive::Disabled)
    .run()
    .await

}
