use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::sync::{Arc, RwLock};

fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:25565")?;
    let arc = Arc::new(RwLock::new(stream));
    let connection_closed = Arc::new(RwLock::new(false));
    arc.read().expect("Could not lock").set_nonblocking(true)?;

    let mut handles = vec![];

    let stream_lock = Arc::clone(&arc);
    let connection_closed_lock = Arc::clone(&connection_closed);
    let read_handle = thread::spawn(move || {
        loop {
            let mut buf: [u8; 1024] = [0; 1024];
            let mut stream = stream_lock.write().expect("Could not lock");

            match stream.read(&mut buf) {
                Ok(0) => {
                    *(connection_closed_lock.write().expect("Could not lock")) = true;
                    break;
                }
                Ok(n) => {
                    // TODO (yash): output buf to GUI instead (printing right now for testing)
                    print!("Server: {}", std::str::from_utf8(&buf[0..n]).unwrap());
                }
                Err(e) => {
                    match e.kind() {
                        std::io::ErrorKind::WouldBlock => continue,
                        _ => {
                            *(connection_closed_lock.write().expect("Could not lock")) = true;
                            break;
                        }
                    }
                }
            };
        }
    });
    handles.push(read_handle);

    let stream_lock = Arc::clone(&arc);
    let connection_closed_lock = Arc::clone(&connection_closed);
    let write_handle = thread::spawn(move || {
        loop {
            let connection_closed = *(connection_closed_lock.read().expect("Could not lock"));

            if !connection_closed {
                let mut string = String::new();

                // TODO (yash): read from GUI instead of stdin
                std::io::stdin().read_line(&mut string).expect("Could not read from stdin");

                let mut stream = stream_lock.write().expect("Could not lock");

                let _ = stream.write(&string.as_bytes());
            }
            else {
                println!("Connection to server closed");
                break;
            }
        }
    });
    handles.push(write_handle);

    for handle in handles {
        handle.join().expect("Error joining threads");
    }

    Ok(())
}