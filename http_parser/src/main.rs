use std::{collections::HashMap, net::{TcpListener, TcpStream}};
use std::io::{BufReader, BufRead};
// struct Http {
//     method: String,
//     Request: String,
//     Version: u32,
//     Header: HashMap<String, String>,
//     Body: Option<String>
// }





fn handle_requst(stream: TcpStream) {
    let mut reader = BufReader::new(stream);

    let mut request_line = String::new();
    if let Err(e) = reader.read_line(&mut request_line) {
        eprintln!("Error reading the request line {e}");
        return;
    }
    let requst_line = request_line.trim_end();
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() != 3 {
        eprintln!("Malformed request line {request_line}");
        return;
    }
    let method = parts[0];
    let path = parts[1];
    let version = parts[2];
    println!("Method: {}, Path: {} Version: {}", method, path, version);

    let mut headers = Vec::new();
    loop {
        let mut line = String::new();
        if let Err(e) = reader.read_line(&mut line) {
            eprintln!("Error reading headers {e}");
            break;
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        headers.push(line.to_string());
    }
    println!("Headers: {:?}", headers);
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to port 8080");
    println!("Listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_requst(stream);
            }
            Err(e) => {
                eprintln!("Connection Failed {e}");
            }
        }
    }
}
