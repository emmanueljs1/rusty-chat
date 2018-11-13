use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565")?;

    let mut streams: Vec<TcpStream> = vec![];

    // accept connections and process them serially
    loop {
        for mut stream in streams.iter() {
           let mut buf: [u8; 1] = [0; 1];
           let mut buffer: String = String::new();

           loop {
               match stream.read(&mut buf) {
                   Ok(0) => break,
                   Ok(_) => {
                       match std::str::from_utf8(&buf) {
                           Ok(string) => buffer.push_str(&string),
                           Err(e) => {
                               println!("Error reading stream: {}", e);
                               break;
                           }
                       }
                   }
                   Err(e) => {
                       println!("Error reading stream: {}", e);
                       break;
                   }
               }
           }

           if !buffer.is_empty() {
               println!("{}", buffer);
           }
        }

        match listener.accept() {
            Ok((stream, _)) => streams.push(stream),
            Err(e) => println!("Error: {}", e),
        }
    }
}
