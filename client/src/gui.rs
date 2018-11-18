use gtk::{
    ContainerExt,
    Entry,
    EntryExt,
    Inhibit,
    Label,
    LabelExt,
    WidgetExt,
    ScrolledWindow,
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
    content: String,
    stream_lock: Arc<RwLock<TcpStream>>,
}

#[derive(Msg)]
pub enum Msg {
    SendMsg,
    Quit,
    Received(Option<String>),
}

pub struct Win {
    model: Model,
    widgets: Widgets,
}

#[derive(Clone)]
pub struct Widgets {
    input: Entry,
    label: Label,
    window: Window,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = SocketAddr;
    type Msg = Msg;

    fn model(_: &Relm<Self>, addr: SocketAddr) -> Model {
        let stream = TcpStream::connect(addr).expect("Cound not connect to server");
        let arc = Arc::new(RwLock::new(stream));

        Model {
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
                let string: String = self.widgets.input.get_text()
                                               .expect("get_text failed")
                                               .chars()
                                               .collect();
                if !string.is_empty() {
                    self.widgets.input.set_text("");
                    let mut stream = self.model.stream_lock.write().expect("Could not lock");
                    /* TODO: instead send a command formatted string */
                    let _ = stream.write(&string.as_bytes());
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

        let vbox = gtk::Box::new(Vertical, 2);

        let scroll_window = ScrolledWindow::new(None, None);
        let label = Label::new(None);
        scroll_window.add(&label);
        vbox.add(&scroll_window);

        let hbox = gtk::Box::new(Horizontal, 0);

        let input = Entry::new();
        hbox.add(&input);

        let button = Button::new_with_label("Send");
        hbox.add(&button);

        vbox.add(&hbox);

        let window = Window::new(WindowType::Toplevel);

        window.add(&vbox);

        window.show_all();

        connect!(relm, button, connect_clicked(_), SendMsg);
        connect!(relm, window, connect_delete_event(_, _), return (Some(Quit), Inhibit(false)));

        Win {
            model,
            widgets: Widgets {
                input,
                label,
                window,
            },
        }
    }
}