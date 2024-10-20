//-------this is for Ch.20 web server----------
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use world_hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    
    // for stream in listener.incoming().take(2)

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });

    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{} {}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

//-------this is for Ch.20 web server----------

// // use mylib::basic::basic;
// // use mylib::factory as test;
// // use mylib::factory::produce_refrigerator::A;
// // use mylib::factory::produce_washing_machine::B::C;

// // extern crate crypto;
// // use crypto::digest::Digest;
// // use crypto::sha3::Sha3;

// use mylib::pro;

// fn main() {
//     // let mut hasher = Sha3::sha3_256();
//     // hasher.input_str("hello world");
//     // println!("hash: {}", hasher.result_str());

//     // let a = A::new();
//     // a.print();
//     // test::produce_refrigerator::produce_re();
//     // C::print_c();

//     // basic::error_mod::run();

//     pro::rc_pointer_mod::lecture_2::run();
// }
