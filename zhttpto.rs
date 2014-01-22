//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::str;
use std::os;

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut cnt: i32     = 0;


fn get_file_content(request: &str) -> (~str, ~str) {
    let split_request: ~[&str] = request.split(' ').collect();
    let page_to_load = match split_request {
        [&"GET", &"/", ..]         => Some(~"./index.html"),    // website home
        [&"GET", custom_page, ..]  => Some(~"." + custom_page), // custom page of the website
        _                          => None                      // unknown request
    };
   
     match page_to_load {
        Some(r) => {
            let p = Path::new(r.clone());
            let mut split_path: ~[&str]  = r.split('.').collect();
            if p.exists() &&  split_path.pop().unwrap() == &"html" // if the current path exist try to open the file and if the file is .html
                && os::getcwd().is_ancestor_of(&os::getcwd().join(&p)) { 
                match File::open(&p) {
                    Some(mut file)  => (~"200 OK", str::from_utf8(file.read_to_end()).unwrap().to_owned()),
                    None            => (~"403 Forbidden", ~"Error 403 You can't be here, can't open this file.")
                }
            } else {  // the file doesn't exist, return an error message
                (~"403 Forbidden", ~" Error 403 You can't be here !")
            }
        },
        None    => (~"400 Bad Request", ~"Error 400 Sorry unknown request") // the request dont'exist, return an error message
    }
}

fn handle_request(request: &str) -> ~[u8] {
   
    let (http_code, html_content) = get_file_content(request);

    let response = 
        &"HTTP/1.1 " + http_code + "\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n" +
        &"number of requests: " + unsafe { cnt.to_str() } + "</br>" + html_content.to_str();  
    response.into_bytes()
}

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();

    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {
                                    println(format!("Received connection from: [{:s}]", pn.to_str())); 
                                    unsafe { cnt += 1; } 
                                },
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str.unwrap()));
            // handle the request
            stream.write(handle_request(request_str.unwrap()));
            println!("Connection terminates.");
        }
    }
}
