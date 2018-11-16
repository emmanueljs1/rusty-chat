use gtk::{ContainerExt, EditableSignals, Entry, EntryExt, Inhibit, Label, LabelExt, WidgetExt, Window, WindowType};
use gtk::Orientation::Vertical;
use relm::{Relm, Update, Widget};

use self::Msg::*;

pub struct Model {
    content: String,
}

#[derive(Msg)]
pub enum Msg {
    Change,
    Quit,
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
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            content: String::new(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Change => {
                self.model.content = self.widgets.input.get_text()
                                                       .expect("get_text failed")
                                                       .chars()
                                                       .collect();
                self.widgets.label.set_text(&self.model.content);
            },
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
        let vbox = gtk::Box::new(Vertical, 0);

        let label = Label::new(None);
        vbox.add(&label);

        let input = Entry::new();
        vbox.add(&input);

        let window = Window::new(WindowType::Toplevel);

        window.add(&vbox);

        window.show_all();

        connect!(relm, input, connect_changed(_), Change);
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