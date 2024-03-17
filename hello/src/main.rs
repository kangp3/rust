use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let status_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Request: {:#?}", status_line);

    let is_root_request = status_line == "GET / HTTP/1.1";
    let response_status = if is_root_request {"200 OK"} else {"404 NOT FOUND"};
    let filename = if is_root_request {"hello.html"} else {"404.html"};

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("HTTP/1.1 {response_status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
