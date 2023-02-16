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

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&mut stream);
  let request_line = String::from(buf_reader.lines().next().unwrap().unwrap());
  
  let req: Vec<&str> = request_line.split(' ').collect();

  // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
  //   ("HTTP/1.1 200 OK", "hello.html")
  // } else {
  //   println!("reqeust: {}", request_line);
  //   ("HTTP/1.1 404 NOT FOUND", "404.html")
  // };

  let (status_line , filename) = if request_line == "GET / HTTP/1.1" {
    ("HTTP/1.1 200 OK" , "hello.html")
  } else if req[0] == "GET" && req[1].chars().nth(0).unwrap() == '/' {
    let mut file_request = req[1];
    let mut file_request_name: &str = file_request.clone();
    if file_request.len() > 0 {
      file_request_name = &file_request[1..file_request.len()];
    }    
    file_request = file_request_name;
    println!("file_request: {:?}",file_request);
    ("HTTP/1.1 200 OK", file_request)
    // ("HTTP/1.1 200 OK" , "hello.html")
  } else {
    println!("reqeust: {}", request_line);
    ("HTTP/1.1 404 NOT FOUND", "404.html")
  };

  let contents = fs::read_to_string(filename).unwrap();
  println!("{contents}");
  let length = contents.len();

  let response = format!(
    "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
  );

  stream.write_all(response.as_bytes()).unwrap();

  // let buf2 = BufReader::new(&mut stream);
  // let http_request: Vec<_> = buf2
  //   .lines()
  //   .map(|result| result.unwrap())
  //   .take_while(|line| !line.is_empty())
  //   .collect();
  // println!("Request: {:#?}", http_request);
}