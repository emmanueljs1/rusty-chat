use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::sync::{Arc, RwLock};

fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:25565")?;
    let arc = Arc::new(RwLock::new(stream));
    arc.read().expect("Could not lock").set_nonblocking(true)?;

    let mut handles = vec![];

    let stream_lock = Arc::clone(&arc);
    let read_handle = thread::spawn(move || {
        loop {
            let mut buf: [u8; 1024] = [0; 1024];
            let mut stream = stream_lock.write().expect("Could not lock");
            match stream.read(&mut buf) {
                Ok(0) => continue,
                Ok(n) => {
                    // TODO (yash): output buf to GUI instead (printing right now for testing)
                    print!("Server: {}", std::str::from_utf8(&buf[0..n]).unwrap());
                }
                Err(_) => continue,
            };
        }
    });
    handles.push(read_handle);

    let stream_lock = Arc::clone(&arc);
    let write_handle = thread::spawn(move || {
        loop {
            let mut stdin_buf = [0; 1024];

            std::io::stdin().read(&mut stdin_buf).expect("Could not read from stdin");
            let mut stream = stream_lock.write().expect("Could not lock");
            stream.write(&stdin_buf).expect("Could not write to stream");
        }
    });
    handles.push(write_handle);

    for handle in handles {
        handle.join().expect("Error joining threads");
    }

    Ok(())
}