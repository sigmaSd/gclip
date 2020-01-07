use atty::{is, Stream};
use gtk::{self, ButtonExt, ContainerExt, WidgetExt};
use std::io::{self, Read};

fn main() {
    let text = if is(Stream::Stdin) {
        // stdin is tty -> stdin is empty, look for an arg
        std::fs::read_to_string(std::env::args().nth(1).expect("No file specified"))
            .expect("Error reading file")
    } else {
        let mut text = String::new();
        io::stdin()
            .read_to_string(&mut text)
            .expect("Error reading stdin");
        text
    };

    paste_to_cp(text);
}

fn paste_to_cp(text: String) {
    gtk::init().expect("Error initializing gtk");

    let cp = gtk::Clipboard::get_default(
        &gdk::Display::get_default().expect("Error getting default display"),
    );
    let cp = cp.expect("Error getting default Clipboard");

    let btn = gtk::Button::new_with_label("copy");
    let win = gtk::Window::new(gtk::WindowType::Toplevel);
    win.add(&btn);
    win.show_all();

    win.connect_delete_event(|_, _| {
        gtk::main_quit();
        gtk::Inhibit(false)
    });

    btn.connect_clicked(move |_| {
        cp.set_text(&text);
        cp.store();
    });

    gtk::main();
}
