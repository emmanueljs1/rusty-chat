extern crate local_ip;
extern crate structopt;
mod server;
mod command;
mod args;

use std::io::prelude::*;
use std::thread;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::clone::Clone;
use structopt::StructOpt;
use server::*;
use command::*;
use args::Opt;

fn main() -> std::io::Result<()> {
    let opt: Opt = Opt::from_args();

    let mut ip;

    if opt.remote {
        ip = local_ip::get().unwrap().to_string();
        ip.push_str(":65535");
    }
    else {
        ip = "127.0.0.1:65535".to_string()
    }

    println!("Clients should use IP address: {}", ip);


    let listener = TcpListener::bind(ip)?;
    let streams = Arc::new(RwLock::new(HashMap::<SocketAddr, TcpStream>::new()));
    let model = Arc::new(RwLock::new(ServerModel::new()));
    let mut handles = vec![];

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("New client: {}", addr);
                let mut write_lock = streams.write().expect("Could not lock");
                write_lock.insert(addr, stream);

                let mut thread_streams = Arc::clone(&streams);
                let mut thread_model = Arc::clone(&model);

                let handle = thread::spawn(move || {
                    let mut just_connected = true;
                    let mut stream_closed = false;
                    
                    let mut locked_server = thread_model.write().expect("Failed to lock server");
                    let user_id = locked_server.register_user();
                    drop(locked_server);

                    loop {
                        let streams = thread_streams.read().expect("Could not lock");

                        let mut buffer: String = String::new();

                        let mut buf: [u8; 1024] = [0; 1024];

                        let mut stream = streams.get(&addr).expect("Stream not found");
                        let _ = stream.set_nonblocking(true);

                        loop {
                            match stream.read(&mut buf) {
                                Ok(0) => {
                                    stream_closed = true;
                                    break;
                                }
                                Ok(n) => buffer.push_str(std::str::from_utf8(&buf[0..n]).unwrap()),
                                Err(e) =>  {
                                    match e.kind() {
                                        std::io::ErrorKind::WouldBlock => (),
                                        _ => {
                                            stream_closed = true;
                                        }
                                    }
                                    break;
                                }
                            };
                        }

                        if stream_closed {
                            break;
                        }

                        if just_connected {
                            for mut stream in streams.values() {
                                let rl_server = thread_model.read().expect("Could not lock server");
                                let mut default_nickname = rl_server.get_nickname(user_id);
                                default_nickname.push_str(&" connected".to_string());
                                let _ = stream.write(default_nickname.as_bytes());
                                let _ = stream.flush();
                                
                                if stream.peer_addr().unwrap() == addr {
                                    let mut welcome_str = "\nWelcome to Rusty Chat, you are ".to_string(); 
                                    welcome_str.push_str(&rl_server.get_nickname(user_id));
                                    let _ = stream.write(welcome_str.as_bytes());
                                    let _ = stream.flush();
                                }
                            }
                        }

                        if !buffer.is_empty() {
                            let cmd: Command = buffer.parse::<Command>().unwrap();

                            let mut wl_server = thread_model.write().expect("Could not lock");
                            let nickname = wl_server.update_with_cmd(&cmd, user_id);
                            drop(wl_server);
                            let cmd_string = cmd.as_msg(nickname);

                            for mut stream in streams.values() {
                                let _ = stream.write_all(&cmd_string.as_bytes());
                                let _ = stream.flush();
                            }
                        }

                        just_connected = false;
                    }

                    let mut streams = thread_streams.write().expect("Could not lock");
                    streams.remove(&addr);

                    let mut server_for_delete = thread_model.write().expect("Could not lock server for deletion");

                    for mut stream in streams.values() {
                        let mut disconnect_msg = server_for_delete.get_nickname(user_id);
                        disconnect_msg.push_str(&" disconnected".to_string());
                        let _ = stream.write(disconnect_msg.as_bytes());
                        let _ = stream.flush();
                    }

                    server_for_delete.remove_user(user_id);
                    println!("Client exited: {}", addr);
                });

                handles.push(handle);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }

    for handle in handles {
        let _ = handle.join();
    }

    Ok(())
}
