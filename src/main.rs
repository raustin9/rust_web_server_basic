use web_server::ThreadPool;
use std::{
  fs,
  io::{prelude::*, BufReader},
  net::{TcpListener, TcpStream},
};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let pool = ThreadPool::new(10);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(|| {
      handle_connection(stream);
    });
  }

  println!("Shutting down");
}

// This handles the HTTP requests going in and out
// Currently cannot handle requesting files that do not contain valid UTF-8 encoded data
// So image, favicon.ico, etc cannot be requested without error
fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&mut stream);
  let request_line = String::from(buf_reader.lines().next().unwrap().unwrap());
  
  let req: Vec<&str> = request_line.split(' ').collect();

  let (status_line , filename) = if request_line == "GET / HTTP/1.1" {
    ("HTTP/1.1 200 OK" , "index.html")
  } else if req[0] == "GET" && req[1].chars().nth(0).unwrap() == '/' {
    let file_request = get_request_file_name(req[1]);
    println!("file_request: {:?}",file_request);
    ("HTTP/1.1 200 OK", file_request)
  } else {
    println!("reqeust: {}", request_line);
    ("HTTP/1.1 404 NOT FOUND", "404.html")
  };

  let contents = fs::read_to_string(filename).unwrap();
  let length = contents.len();

  let response = format!(
    "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
  );

  stream.write_all(response.as_bytes()).unwrap();
}

// Remove the first '/' from the requested file string
// ex: /index.js -> index.js
fn get_request_file_name(file_request: &str) -> &str {
  let file_request_name: &str = if file_request.len() > 0 {
    &file_request[1..file_request.len()]
  } else {
    file_request
  };

  file_request_name
}