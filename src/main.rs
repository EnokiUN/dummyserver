use std::{
    collections::HashMap,
    env, error,
    io::{BufRead, BufReader, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
};

#[allow(dead_code)]
#[derive(Debug)]
struct Request {
    peer_addr: Option<SocketAddr>,
    method: String,
    route: String,
    http: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

fn main() {
    let address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let listener = TcpListener::bind(&address)
        .unwrap_or_else(|_| panic!("Could not start TCP server at {}", address));
    for stream in listener.incoming() {
        thread::spawn(move || match stream {
            Ok(stream) => {
                if let Err(err) = handle_request(stream) {
                    eprintln!("Could not extract data from request: {}", err);
                }
            }
            Err(err) => {
                eprintln!("Encountered error with connection: {}", err);
            }
        });
    }
}

fn handle_request(mut stream: TcpStream) -> Result<(), Box<dyn error::Error>> {
    let mut buf = BufReader::new(&mut stream);
    let mut first = String::new();
    let mut content_length = 0;
    let mut body: Option<String> = None;
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut headers_done = false;
    buf.read_line(&mut first)?;
    let mut first = first.split(' ');

    loop {
        if headers_done {
            if content_length == 0 {
                break;
            }
            let mut body_buf = vec![0; content_length];
            buf.read_exact(&mut body_buf)?;
            body = Some(String::from_utf8(body_buf.to_vec())?);
            break;
        }
        let mut l = String::new();
        buf.read_line(&mut l)?;
        l.pop();
        l.pop();
        if l.is_empty() {
            headers_done = true;
            continue;
        }
        let (header, value) = l.split_once(": ").ok_or("Malformed header syntax")?;
        if header == "Content-Length" {
            content_length = value.parse()?;
        }
        headers.insert(header.to_string(), value.to_string());
    }

    stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;

    let method = first.next().ok_or("Malformed HTTP request")?.to_string();
    let route = first.next().ok_or("Malformed HTTP request")?.to_string();
    let mut http = first.next().ok_or("Malformed HTTP request")?.to_string();
    http.pop();
    http.pop();

    let request = Request {
        peer_addr: stream.peer_addr().ok(),
        method,
        route,
        http,
        headers,
        body,
    };
    println!("{:#?}", request);

    Ok(())
}
