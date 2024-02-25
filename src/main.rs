/*!
    A very simple application that shows your name in a message box.
    Unlike `basic_d`, this example uses layout to position the controls in the window
*/

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use handlebars::Handlebars;
use std::{cell::RefCell, borrow::Borrow, thread::sleep, time::Duration};
use std::ops::Deref;
use std::rc::Rc;

pub mod process;
use process::{
    caddy::{self},
    php::{self},
};

const APP_ID: &'static str = "org.gtk_rs.HelloWorld2";
const BLIS_VERSION: &'static str = "4.0";

pub struct BlisApp {
    php: php::PhpFpm,
    caddy: caddy::Caddy,
}

impl BlisApp {
    pub fn new() -> BlisApp {
        let mut handlebars = Handlebars::new();

        handlebars
            .register_template_file("php.ini", "config/php.ini.hbs")
            .expect("Could not load php.ini.hbs");

        handlebars.set_strict_mode(true);

        BlisApp {
            php: php::PhpFpm::new(handlebars),
            caddy: caddy::Caddy::new(),
        }
    }

    pub fn start(&mut self) {
        self.php.spawn();
        sleep(Duration::new(1, 0));
        self.caddy.spawn();
    }

    pub fn stop(&self) {
        self.caddy.stop();
        self.php.stop();
    }
}

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    let mut appsvc = BlisApp::new();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    appsvc.start();

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Present window
    window.present();
}

fn teardown(app: &Application) {
    // TODO
}
