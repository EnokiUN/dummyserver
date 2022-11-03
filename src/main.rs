use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
    thread,
};

#[allow(dead_code)]
#[derive(Debug)]
struct Request {
    method: String,
    route: String,
    headers: Vec<String>,
    body: Option<String>,
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            let mut buf = BufReader::new(&mut stream);
            let mut first = String::new();
            let mut content_length = 0;
            let mut body: Option<String> = None;
            let mut headers: Vec<String> = Vec::new();
            let mut headers_done = false;
            buf.read_line(&mut first).unwrap();
            let mut first = first.split(" ");
            loop {
                if headers_done {
                    if content_length == 0 {
                        break;
                    }
                    let mut body_buf = vec![0; content_length];
                    buf.read_exact(&mut body_buf).unwrap();
                    body = Some(String::from_utf8(body_buf.to_vec()).unwrap());
                    break;
                }
                let mut l = String::new();
                buf.read_line(&mut l).unwrap();
                l.pop();
                l.pop();
                if l.is_empty() {
                    headers_done = true;
                    continue;
                }
                if l.starts_with("Content-Length") {
                    content_length = l.split(" ").nth(1).unwrap().parse().unwrap();
                }
                headers.push(l);
            }
            let request = Request {
                method: first.next().unwrap().to_string(),
                route: first.next().unwrap().to_string(),
                headers,
                body,
            };
            println!("{:#?}", request);
            stream
                .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                .unwrap();
        });
    }
}
