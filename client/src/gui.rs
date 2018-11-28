use gtk::{
    AdjustmentExt,
    Align,
    BoxExt,
    ContainerExt,
    Dialog,
    DialogExt,
    DialogFlags,
    Entry,
    EntryExt,
    GtkWindowExt,
    Inhibit,
    Label,
    LabelExt,
    WidgetExt,
    ResponseType,
    ScrolledWindow,
    ScrolledWindowExt,
    Window,
    WindowType,
    Button,
    ButtonExt
};
use gtk::Orientation::{Horizontal, Vertical};
use relm::{Relm, Update, Widget};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::thread;
use std::sync::{Arc, RwLock};
use futures::sync::mpsc::channel;
use self::Msg::*;

pub struct Model {
    relm: Relm<Win>,
    content: String,
    stream_lock: Arc<RwLock<TcpStream>>,
}

#[derive(Msg)]
pub enum Msg {
    ScrollDown,
    SendMsg,
    Quit,
    Received(Option<String>),
    OpenUsernameDialog,
    ChangeUsername(Option<String>, Dialog),
    CloseDialog
}

pub struct Win {
    model: Model,
    widgets: Widgets,
}

#[derive(Clone)]
pub struct Widgets {
    messages: ScrolledWindow,
    message_input: Entry,
    label: Label,
    window: Window,
    // username_input: Entry,
    // username_dialog: Dialog,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = SocketAddr;
    type Msg = Msg;

    fn model(relm: &Relm<Self>, addr: SocketAddr) -> Model {
        let stream = TcpStream::connect(addr).expect("Cound not connect to server");
        let arc = Arc::new(RwLock::new(stream));

        Model {
            relm: relm.clone(),
            content: String::new(),
            stream_lock: arc,
        }
    }

    fn subscriptions(&mut self, relm: &Relm<Self>) {
        let (mut sender, receiver) = channel::<Option<String>>(1);

        let stream_lock = Arc::clone(&self.model.stream_lock);
        thread::spawn(move || {
            loop {
                let mut buf: [u8; 1024] = [0; 1024];
                let mut stream = stream_lock.write().expect("Could not lock");
                stream.set_nonblocking(true).expect("Could not set set_nonblocking");

                match stream.read(&mut buf) {
                    Ok(0) => {
                        sender.try_send(None).expect("Could not send");
                        break;
                    }
                    Ok(n) => {
                        let string = std::str::from_utf8(&buf[0..n]).unwrap().to_string();
                        sender.try_send(Some(string)).expect("Could not send");
                    }
                    Err(e) => {
                        match e.kind() {
                            std::io::ErrorKind::WouldBlock => continue,
                            _ => {
                                sender.try_send(None).expect("Could not send");
                                break;
                            }
                        }
                    }
                };
            }
        });

        relm.connect_exec_ignore_err(receiver, Received);
    }

    fn update(&mut self, event: Msg) {
        match event {
            Received(string_opt) => {
                match string_opt {
                    Some(string) => {
                        self.model.content += "\n";
                        self.model.content += &string;
                        self.widgets.label.set_text(&self.model.content);
                    }
                    None => gtk::main_quit(),
                }
            }
            SendMsg => {
                let mut string: String = self.widgets.message_input.get_text()
                                               .expect("get_text failed")
                                               .chars()
                                               .collect();
                println!("{:?}", string.as_str());
                if !string.is_empty() {
                    self.widgets.message_input.set_text("");
                    let mut stream = self.model.stream_lock.write().expect("Could not lock");
                    /* TODO: instead send a command formatted string */
                    string = "/msg ".to_string() + &string;
                    let _ = stream.write(&string.as_bytes());
                }
            }
            ScrollDown => {
                let scroll_pos = self.widgets.messages.get_vadjustment().unwrap();
                scroll_pos.set_value(scroll_pos.get_upper());
            }
            OpenUsernameDialog => {
                // Change username dialog
                let username_dialog = Dialog::new_with_buttons(
                                Some("Change Username"),
                                Some(&self.widgets.window),
                                DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT,
                                &[("Change", ResponseType::Apply.into()), 
                                  ("Cancel", ResponseType::Cancel.into())]
                            );
                let username_input = Entry::new();
                username_dialog.get_content_area().add(&username_input);
                username_dialog.show_all();
                connect!(self.model.relm, username_dialog, connect_response(_, change_response), 
                    ChangeUsername(username_input.get_text()));
                // connect!(self.model.relm, username_button, connect_clicked(_), ChangeUsername(username_input.get_text()));
                // let result = username_dialog.run();
                // if result == 

            }
            CloseDialog => {
                // self.widgets.username_dialog.destroy();
                println!("Dialog closed");
            }
            ChangeUsername(new_username_opt, username_dialog) => {
                // let new_username: String = self.widgets.username_input.get_text()
                //                                .expect("get_text failed")
                //                                .chars()
                //                                .collect();

                match new_username_opt {
                    Some(new_username) => {
                        if !new_username.is_empty() {
                            // self.widgets.username_input.set_text("");
                            // TODO: send change username command to server
                            println!("{:?}", new_username);
                            // TODO: figure out how to close dialog
                        }
                    },
                    None => gtk::main_quit(), 
                }
            }
            Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        /* TODO (yash):
            - change GUI to be a _real_ GUI
            - add option to change nickname and (possibly?) to change text color */

        let window = Window::new(WindowType::Toplevel);
        let vbox = gtk::Box::new(Vertical, 2);

        // Conversation History
        let messages = ScrolledWindow::new(None, None);
        messages.set_min_content_height(400);
        let label = Label::new(None);
        label.set_valign(Align::Start);
        label.set_halign(Align::Start);
        messages.add(&label);
        vbox.pack_start(&messages, true, true, 0);

        // Change username button
        let username_box = gtk::Box::new(Horizontal, 1);
        // let username_input = Entry::new();
        // username_box.add(&username_input);
        let username_button = Button::new_with_label("Change username");
        username_box.add(&username_button);
        vbox.pack_end(&username_box, false, false, 0);

        // Message Input
        let message_box = gtk::Box::new(Horizontal, 1);
        let message_input = Entry::new();
        message_input.set_width_chars(30);
        message_input.set_property_show_emoji_icon(true);
        message_box.pack_start(&message_input, true, true, 0);
        let button = Button::new_with_label("Send");
        message_box.pack_end(&button, false, false, 0);
        vbox.pack_end(&message_box, false, false, 0);

        // TODO: figure out how to add username to window title
        window.set_title("Chat");
        window.add(&vbox);
        window.show_all();

        // let change_response: i32 = ResponseType::Apply.into();

        connect!(relm, messages.get_vadjustment().unwrap(), connect_property_upper_notify(_), ScrollDown);
        connect!(relm, message_input, connect_activate(_), SendMsg);
        connect!(relm, button, connect_clicked(_), SendMsg);
        connect!(relm, window, connect_delete_event(_, _), return (Some(Quit), Inhibit(false)));
        // connect!(relm, username_button, connect_clicked(_), ChangeUsername);
        connect!(relm, username_button, connect_clicked(_), OpenUsernameDialog);
        // connect!(relm, username_dialog, connect_response(_, change_response), ChangeUsername);
        // connect!(relm, username_dialog, connect_close(_), CloseDialog);

        Win {
            model,
            widgets: Widgets {
                messages,
                message_input,
                label,
                window,
                // username_input,
                //username_dialog,
            },
        }
    }
}
