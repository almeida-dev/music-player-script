// to install gtk3 in debian op distributions:
// sudo apt-get install libgtk-3-dev

extern crate gio;
extern crate gtk;

use std::env;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{Application, ApplicationWindow, GtkWindowExt, WidgetExt};

fn main() {
    let application = Application::new("com.github.rust-by-example", ApplicationFlags::empty())
        .expect("Application initialization failed");
    application.connect_startup(|application| {
        let window = ApplicationWindow::new(&application);
        window.set_title("MUSIC_PLAYER");
        window.show();
    });
    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}
