extern crate gtk;
extern crate futures;
extern crate pango;
extern crate structopt;
#[macro_use] extern crate relm;
#[macro_use] extern crate relm_derive;

mod gui;
mod args;

use gui::Win;
use args::Opt;
use relm::Widget;
use std::net::ToSocketAddrs;
use std::io::{Error, ErrorKind};
use structopt::StructOpt;

fn main() -> std::io::Result<()> {
    let opt: Opt = Opt::from_args();

    let addr_str = {
        match opt.ip_addr {
            None => "127.0.0.1:65535".to_string(),
            Some(str) => str,
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
