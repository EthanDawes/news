use chrono::Utc;
use percent_encoding::percent_decode_str;
use simple_http_parser::request::{HTTPMethod, Request};
use std::{
    fs::{self, OpenOptions},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const SUBSCRIPTIONS_PATH: &str = "../../recipients.csv";
const ACCESS_LOG_PATH: &str = "../../access.csv";
const PIXEL_PATH: &str = "../../pixel.png";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        match handle_connection(&stream) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("{}", err.to_string());
                send_response(
                    &stream,
                    "500 SERVER ERROR",
                    "text/plain",
                    err.to_string().as_bytes(),
                )
                .unwrap()
            }
        }
    }
}

fn bad_request(stream: &TcpStream) -> std::io::Result<()> {
    send_response(stream, "400 BAD REQUEST", "text/plain", b"bad request")
}

fn invalid_email(email: &str) -> bool {
    if let Some(at_pos) = email.find('@') {
        if let Some(dot_pos) = email[at_pos..].find('.') {
            // Ensure there's at least one character between '@' and '.'
            if dot_pos > 1 {
                return false; // Email is valid
            }
        }
    }
    true // Email is invalid
}

fn handle_connection(mut stream: &TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let raw_request = String::from_utf8_lossy(&buffer);

    let req = Request::from(&raw_request)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    if req.method != HTTPMethod::GET {
        return bad_request(stream);
    }

    let path_parts: Vec<&str> = req.path.split('/').collect();

    println!("{} {}", req.method.as_str(), req.path);

    match path_parts[1] {
        "hello" => handle_hello(stream, path_parts, &req),
        "subscribe" => handle_subscribe(stream, path_parts),
        "unsubscribe" => handle_unsubscribe(stream, path_parts),
        _ => send_response(stream, "404 NOT FOUND", "text/plain", b"not found"),
    }
}

fn handle_hello(stream: &TcpStream, path_parts: Vec<&str>, req: &Request) -> std::io::Result<()> {
    // /hello/<info>
    if path_parts.len() != 3 {
        return bad_request(stream);
    }
    let info = percent_decode_str(path_parts[2]).decode_utf8_lossy();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(ACCESS_LOG_PATH)?;

    let now = Utc::now();
    let iso_datetime = now.to_rfc3339();

    let peer_address = stream.peer_addr()?.ip();
    let mut ip = &peer_address.to_string();
    if peer_address.is_loopback() {
        if let Some(forwarded) = req.headers.get("Forwarded") {
            ip = forwarded;
        }
        if let Some(forwarded) = req.headers.get("X-Forwarded-For") {
            ip = forwarded;
        }
    }
    let blank_str = String::new();
    let user_agent = req.headers.get("User-Agent").unwrap_or(&blank_str);

    writeln!(file, "{},{},{},{}", iso_datetime, ip, user_agent, info)?;

    let pixel_data = fs::read(PIXEL_PATH).unwrap_or_else(|_| Vec::new());
    send_response(stream, "200 OK", "image/png", &pixel_data)
}

fn handle_subscribe(stream: &TcpStream, path_parts: Vec<&str>) -> std::io::Result<()> {
    // /subscribe/<email>/<name>
    if path_parts.len() != 4 {
        return bad_request(stream);
    }
    let email = path_parts[2];
    let name = path_parts[3];
    if invalid_email(email) {
        return bad_request(stream);
    }

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
    if invalid_email(email) {
        return bad_request(stream);
    }

    let file = OpenOptions::new()
        .read(true)
        .open(SUBSCRIPTIONS_PATH)
        .unwrap();
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
        send_response(
            stream,
            "404 NOT FOUND",
            "text/plain",
            b"Subscription not found",
        )
    }
}

fn send_response(
    mut stream: &TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> std::io::Result<()> {
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
