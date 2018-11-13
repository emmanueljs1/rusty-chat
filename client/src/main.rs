use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:25565")?;
    stream.write(&['h' as u8, 'e' as u8, 'y' as u8])?;
    Ok(())
}
