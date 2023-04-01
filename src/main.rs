use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    thread,
    time::Duration,
};

use tcprust::ThreadPool;

static _PAGES: &str = "scr/pages";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1703").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let response = create_response(&stream);
    stream.write_all(response.as_bytes()).unwrap();
}

fn create_response(mut stream: &std::net::TcpStream) -> String {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/pages/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "src/pages/hello.html" )
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "src/pages/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}
