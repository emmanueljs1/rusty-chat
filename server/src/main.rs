use std::io::prelude::*;
use std::thread;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::clone::Clone;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565")?;
    let streams = Arc::new(RwLock::new(HashMap::<SocketAddr, TcpStream>::new()));
    let mut handles = vec![];

    /* TODO (megan):
     let model = ServerModel::new();

     should have:
        registerUser() returns id of new user
        getNickname(id) gets nickname of user
        changeNickname(id) sets nickname of user
     */

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("New client: {}", addr);
                let mut write_lock = streams.write().expect("Could not lock");
                write_lock.insert(addr, stream);

                let mut thread_streams = Arc::clone(&streams);

                let handle = thread::spawn(move || {
                    let mut just_connected = true;
                    let mut stream_closed = false;

                    /* TODO (megan):
                        let user_id = model.registerUser();
                    */

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
                                        _ => stream_closed = true,
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
                                // TODO (megan): change 'User' to model.getNickname(user_id)
                                let _ = stream.write(&"User connected\n".to_string().as_bytes());
                                let _ = stream.flush();
                            }
                        }

                         if !buffer.is_empty() {
                            /* TODO (megan):
                                -- change buffer from /<cmd> <arguments> to a Command object by implementing
                                FromStr trait for Command

                                let cmd: Command = buffer.parse::<Command>();
                                let username = model.getNickname(user_id);

                                -- also implement updateServerModel(server_model, sender_id) method that updates
                                server model with respect to command
                                    e.g. NicknameCommand updateServerModel calls server_model changeNickname

                                cmd.update_server_model(server_model, user_id);

                                -- also implement `as_message` that takes in a nickname and prints out
                                the command as a chat server message
                                e.g. NicknameCommand => "User0 has changed nickname to Megan"
                                     MsgCommand with msg="hello!" => "User0: hello!"

                                let cmd_string = cmd.as_message(username);
                            */

                             for mut stream in streams.values() {
                                 // TODO: change &buffer to &cmd_string
                                let _ = stream.write_all(&buffer.as_bytes());
                                let _ = stream.flush();
                             }
                         }

                         just_connected = false;
                    }

                    let mut streams = thread_streams.write().expect("Could not lock");
                    streams.remove(&addr);
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
