// #![feature(mutex_unlock)]
// extern crate redis;
// use redis::Commands;
// use redis::cmd;

// use std::process::{Command, Stdio};
// use threadpool::ThreadPool;
use tiny_keccak::Sha3;
use tiny_keccak::Hasher;
use rand::Rng;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::sync::{Arc, Mutex};
use std::thread;
// ntex
use std::{env, io, str, fmt};
// use log::info;
// use ntex::http::header::HeaderValue;
// use ntex::http::{HttpService, Response};
// use ntex::{server::Server, time::Seconds, util::Ready};
use ntex::time::Seconds;
// use std::fmt::Write as FmtWrite;
use ntex::http;
use ntex::web::{self, middleware, App, HttpServer, HttpResponse};
// use clap::{Parser, Subcommand, ColorChoice, value_parser, Args};
use clap::{Parser, ColorChoice, ArgAction};
// use clap::{Parser, Subcommand, ValueEnum};
use owo_colors::{OwoColorize, Stream};
// use std::cell::Cell;
use std::io::Write;
// use chrono::offset::Utc;
use chrono::offset::Local;
use chrono::DateTime;
use serde::{Deserialize, Serialize};


const VERSION: &str = "0.2.3";
const MAX_RUN: u64 = 1000000000000000000;
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~/|-_,.?";
const PASSWORD_LEN: usize = 10000000;
static LOG_LEVEL_NAMES: [&str; 6] = ["off", "error", "warn", "info", "debug", "trace"];
// const REDIS_URL: &str = "redis://:111111@127.0.0.1/";
// type MyLogLevel = LevelFilter;

#[derive(Debug, Serialize, Deserialize)]
struct JsonResult {
    counter: u64,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy)]
enum Level {
   /// A level lower than all log levels.
   Off,
   /// Corresponds to the `Error` log level.
   Error,
   /// Corresponds to the `Warn` log level.
   Warn,
   /// Corresponds to the `Info` log level.
   Info,
   /// Corresponds to the `Debug` log level.
   Debug,
   /// Corresponds to the `Trace` log level.
   Trace,
}

impl Level {
    fn as_str(&self) -> &'static str {
        LOG_LEVEL_NAMES[*self as usize]
    }
}

impl fmt::Display for Level {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(self.as_str())
    }
}

#[derive(Debug, Parser)]
#[command(version=VERSION, author="Jack Xu")]
// #[command(setting = AppSettings::ColoredHelp)]
struct Cli {
    #[arg(short,long,value_name="Max Run", default_value_t=MAX_RUN)]
    max_run: u64,
    #[arg(short,long,value_name="PASSWORD LEN", default_value_t=PASSWORD_LEN)]
    pass_len: usize,
    #[arg(short,long,value_name="Reader interval(ms)", default_value_t=1000u64)]
    interval: u64,
    #[arg(
        short,
        long,
        // 一定要带等号才认
        // require_equals = true,
        value_name = "WHEN",
        // num_args = 0..=2,
        default_value_t = ColorChoice::Auto,
        // default_missing_value = "auto",
        // value_enum
    )]
    color: ColorChoice,
    #[arg(
        short,
        long,
        // require_equals = true,
        value_name = "LogLevel",
        // num_args = 0..=5,
        default_value_t = Level::Error,
        // default_missing_value = "auto",
        // value_enum
    )]
    log_level: Level,
    #[arg(short,long,value_name="Forever running", default_value_t=false, action=ArgAction::SetTrue, help="default false")]
    forever: bool
}


#[derive(Debug, Clone)]
struct MyState {
    counter: Arc<Mutex<u64>>,
}

fn task(t: usize, max_run: u64, pass_len: usize, locker: Arc<Mutex<u64>>, forever: bool) {
// fn task(t: usize) {
    // *num += 1;
    // println!("{}: {}", t, *c.lock().unwrap());
    // let client = redis::Client::open(REDIS_URL).unwrap();
    // let mut con = client.get_connection().unwrap();
    let run = || {
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
        let mut num = locker.lock().unwrap();
        *num += 1;
    };

    let mut s = Vec::new();
    
    if forever {
        write!(&mut s, "Start task {} - run forever\n", t.if_supports_color(Stream::Stdout, |text| text.red())).unwrap();
        print_with_time(str::from_utf8(&s).unwrap());
        loop {
            run();
        }
    } else {
        let mut n: u64 = 0;
        write!(&mut s, "Start task {} - run max = {}\n", t.if_supports_color(Stream::Stdout, |text| text.red()), max_run).unwrap();
        print_with_time(str::from_utf8(&s).unwrap());
        while n < max_run {
            n += 1;
            run();
        }
    }
    
    
}

async fn http_index(st: web::types::State<MyState>) -> HttpResponse {
// async fn http_index(st: web::types::State<MyState>) -> i32 {
    let c = st.counter.lock().unwrap();
    // println!("{}", *c);
    //let s = c.to_string();
    //HttpResponse::Ok().body(s)
    HttpResponse::Ok().json(&JsonResult{counter: *c})
}

fn print_with_time(s: &str) {
    let now = SystemTime::now();
    let datetime: DateTime<Local> = now.into();
    print!("[{}] {}", datetime.format("%Y/%m/%d %T%.3f"), s);
    io::stdout().flush().unwrap();
}

#[ntex::main]
async fn main() -> io::Result<()> {
    let args = Cli::parse();
    // println!("{:?}", args);
    // init color
    match args.color {
        ColorChoice::Always => owo_colors::set_override(true),
        ColorChoice::Auto => {}
        ColorChoice::Never => owo_colors::set_override(false),
    }
    // env::set_var("RUST_LOG", "ntex=trace");
    let mut ntex_log_str = Vec::new();
    write!(&mut ntex_log_str, "ntex={}", args.log_level).unwrap();

    env::set_var("RUST_LOG", str::from_utf8(&ntex_log_str).unwrap());
    // println!("System up");
    
    env_logger::init();

    let mut s = Vec::new();
    write!(
        &mut s, 
        "{}, Version {}, LogLevel {}\n", 
        "System up".if_supports_color(Stream::Stdout, |text| text.bright_green()), 
        VERSION,
        str::from_utf8(&ntex_log_str).unwrap()
    ).unwrap();
    print_with_time(str::from_utf8(&s).unwrap());

    // clear data
    // let client = redis::Client::open(REDIS_URL).unwrap();
    // let mut con = client.get_connection().unwrap();
    //let _rt = con.del("counter").unwrap();
    // println!("clean redis");
    // redis::cmd("DEL").arg("counter").query::<i32>(&mut con).unwrap();
    // drop(con);
    // drop(client);
    // sleep(Duration::from_secs(1));

    let cpus = num_cpus::get();
    s = Vec::new();
    write!(&mut s, "cpu number: {}\n", cpus.if_supports_color(Stream::Stdout, |text| text.red())).unwrap();
    print_with_time(str::from_utf8(&s).unwrap());
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
                let mut s = Vec::new();
                write!(&mut s, "Start counter with interval {}(ms)\n", args.interval).unwrap();
                print_with_time(str::from_utf8(&s).unwrap());
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
                    // let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                    // let now = SystemTime::now();
                    // let datetime: DateTime<Local> = now.into();
                    s = Vec::new();
                    write!(&mut s, "Counter: {}\r", c.if_supports_color(Stream::Stdout, |text| text.green())).unwrap();
                    print_with_time(str::from_utf8(&s).unwrap());
                    // print!("[{}] Counter: {}\r", now.as_millis(), c.if_supports_color(Stream::Stdout, |text| text.green()));
                    // print!("[{}] Counter: {}\r", datetime.format("%Y/%m/%d %T%.3f"), c.if_supports_color(Stream::Stdout, |text| text.green()));
                    // io::stdout().flush().unwrap();
                    // println!("[{}] Counter: {}", now.as_millis(), c.if_supports_color(Stream::Stdout, |text| text.green()));
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
                task(i, args.max_run, args.pass_len, locker, args.forever);
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
    

    s = Vec::new();
    write!(&mut s, "{}\n", "Start Ntex Server".if_supports_color(Stream::Stdout, |text| text.bright_green())).unwrap();
    print_with_time(str::from_utf8(&s).unwrap());

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
