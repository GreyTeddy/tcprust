use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1703").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let response = create_response(&stream);
        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn create_response(mut stream: &std::net::TcpStream) -> String {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "Get / HTTP/1.1" {
        ("HTTP/1.1 200 OK","hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND","404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}