//! Simple HTTP server.
//!
//! Benchmark with [Apache HTTP server benchmarking tool](https://httpd.apache.org/docs/2.4/programs/ab.html):
//!
//! ```
//! ab -n 5000 -c 20 http://X.X.X.X:5555/
//! ```
//make A=apps/net/httpserver ARCH=aarch64 LOG=info SMP=4 BLK=y run NET=y
#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs::{self, FileType, File};
use axstd::vec::Vec;
use axstd::string::String;
const LOCAL_IP: &str = "0.0.0.0";
const LOCAL_PORT: u16 = 5555;
use std::fs::read;
fn gethtml()->String{
    let mut file = File::open("/html/index.html");
    if file.is_err() {
        println!("file open error");
        return String::from("file open error")

    }
    println!("file open success");
    let mut contents:Vec<u8> = vec![0;65536];
    file.unwrap().read_to_end(&mut contents);
    let s: String = contents.iter().map(|&c| c as char).collect();
    s
}

macro_rules! header {
    () => {
        "\
HTTP/1.1 200 OK\r\n\
Content-Type: text/html\r\n\
Content-Length: {}\r\n\
Connection: close\r\n\
\r\n\
{}"
    };
}

const CONTENT: &str = r#"<html>
<head>
  <title>Hello, ArceOS</title>
</head>
<body>
  <center>
    <h1>Hello, <a href="https://github.com/rcore-os/arceos">ArceOS</a></h1>
  </center>
  <hr>
  <center>
    <i>Powered by <a href="https://github.com/rcore-os/arceos/tree/main/apps/net/httpserver">ArceOS example HTTP server</a> v0.1.0</i>
  </center>
</body>
</html>
"#;


macro_rules! info {
    ($($arg:tt)*) => {
        match option_env!("LOG") {
            Some("info") | Some("debug") | Some("trace") => {
                print!("[INFO] {}\n", format_args!($($arg)*));
            }
            _ => {}
        }
    };
}

fn http_server(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 4096];
    let _len = stream.read(&mut buf)?;
    let content=gethtml();
    let response = format!(header!(), content.len(), content);
    stream.write_all(response.as_bytes())?;

    Ok(())
}

fn accept_loop() -> io::Result<()> {
    let listener = TcpListener::bind((LOCAL_IP, LOCAL_PORT))?;
    println!("listen on: http://{}/", listener.local_addr().unwrap());

    let mut i = 0;
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                info!("new client {}: {}", i, addr);
                thread::spawn(move || match http_server(stream) {
                    Err(e) => info!("client connection error: {:?}", e),
                    Ok(()) => info!("client {} closed successfully", i),
                });
            }
            Err(e) => return Err(e),
        }
        i += 1;
    }
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Hello, ArceOS HTTP server!");
    gethtml();

    accept_loop().expect("test HTTP server failed");
}
