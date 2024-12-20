use std::{
    fs::{self, OpenOptions}, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}
};

const SUBSCRIPTIONS_PATH: &str = "recipients.csv";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        match handle_connection(&stream) {
            Ok(_) => {},
            Err(_) => send_response(&stream, "500 SERVER ERROR", "text/plain", b"server error").unwrap(),
        }
    }
}

fn bad_request(stream: &TcpStream) -> std::io::Result<()> {
    send_response(stream, "400 BAD REQUEST", "text/plain", b"bad request")
}

fn handle_connection(stream: &TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(stream);
    let header = buf_reader.lines().next().unwrap().unwrap_or("".to_owned());
    let http_request: Vec<&str> = header.split(' ').collect();

    if http_request.len() != 3 || http_request[0] != "GET" {
        return bad_request(stream);
    }

    let path_parts: Vec<&str> = http_request[1].split('/').collect();

    println!("Request: {}", header);

    match path_parts[1] {
        "hello" => handle_hello(stream, path_parts),
        "subscribe" => handle_subscribe(stream, path_parts),
        "unsubscribe" => handle_unsubscribe(stream, path_parts),
        _ => send_response(stream, "404 NOT FOUND", "text/plain", b"not found"),
    }
}

fn handle_hello(stream: &TcpStream, path_parts: Vec<&str>) -> std::io::Result<()> {
    // /hello/<info>
    if path_parts.len() != 3 {
        return bad_request(stream);
    }
    let info = path_parts[2];

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("access.csv")?;

    writeln!(file, "{}", info)?;

    let pixel_data = fs::read("pixel.png").unwrap_or_else(|_| Vec::new());
    send_response(stream, "200 OK", "image/png", &pixel_data)
}

fn handle_subscribe(stream: &TcpStream, path_parts: Vec<&str>) -> std::io::Result<()> {
    // /unsubscribe/email/name
    if path_parts.len() != 4 {
        return bad_request(stream);
    }
    let email = path_parts[2];
    let name = path_parts[3];

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(SUBSCRIPTIONS_PATH)?;

    writeln!(file, "{},{}", email, name)?;

    send_response(stream, "200 OK", "text/plain", b"Subscription added")
}

fn handle_unsubscribe(stream: &TcpStream, path_parts: Vec<&str>) -> std::io::Result<()> {
    // /unsubscribe/email
    if path_parts.len() != 3 {
        return bad_request(stream);
    }
    let email = path_parts[2];

    let file = OpenOptions::new().read(true).open(SUBSCRIPTIONS_PATH).unwrap();
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    let mut found = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if !line.starts_with(&email) {
            lines.push(line);
        } else {
            found = true;
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(SUBSCRIPTIONS_PATH)
        .unwrap();

    for line in lines {
        writeln!(file, "{}", line).unwrap();
    }

    if found {
        send_response(stream, "200 OK", "text/plain", b"Unsubscribed successfully")
    } else {
        send_response(stream, "404 NOT FOUND", "text/plain", b"Subscription not found")
    }
}

fn send_response(mut stream: &TcpStream, status: &str, content_type: &str, body: &[u8]) -> std::io::Result<()> {
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        status,
        content_type,
        body.len()
    );

    stream.write_all(response.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()
}
