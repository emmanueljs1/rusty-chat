extern crate gtk;
extern crate futures;
#[macro_use] extern crate relm;
#[macro_use] extern crate relm_derive;

mod gui;

use gui::Win;
use relm::Widget;
use std::env;
use std::net::ToSocketAddrs;
use std::io::{Error, ErrorKind};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        let addr_str = {
            if args.len() == 2 {
                args[1].clone()
            }
            else {
                "127.0.0.1:65535".to_string()
            }
        };

        match addr_str.to_socket_addrs() {
            Ok(mut socket_addrs) => {
                match socket_addrs.next() {
                    Some(socket_addr) => {
                        match Win::run(socket_addr) {
                            Ok(()) => Ok(()),
                            Err(()) => Err(Error::new(ErrorKind::Other, "Could not run GUI")),
                        }
                    }
                    None => Err(Error::new(ErrorKind::InvalidInput, "Invalid address")),
                }
            }
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
    else {
        Err(Error::new(ErrorKind::InvalidInput, "Too many or too few arguments"))
    }
}
