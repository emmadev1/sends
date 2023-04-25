use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk.sends";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button1 = Button::builder().label("ola").build();

    button1.connect_clicked(|button1| {
        button1.set_label("adio");
    });

    let window1 = ApplicationWindow::builder()
        .application(app)
        .title("Sends")
        .child(&button1)
        .build();

    window1.present();
}