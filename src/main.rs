/*!
    A very simple application that shows your name in a message box.
    Unlike `basic_d`, this example uses layout to position the controls in the window
*/

use gtk::glib::ExitCode;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use handlebars::Handlebars;
use webbrowser;

pub mod process;
use process::{
    caddy::{self},
    mysqld::{self},
    php::{self},
};

const APP_ID: &'static str = "edu.gatech.cc.blis";
const BLIS_VERSION: &'static str = "4.0";

pub struct BlisApp {
    caddy: caddy::Caddy,
    mysqld: mysqld::Mysqld,
    php: php::PhpFpm,
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
            mysqld: mysqld::Mysqld::new(),
        }
    }

    pub fn start(&mut self) {
        self.mysqld.spawn();
        self.php.spawn();
        self.caddy.spawn();
    }

    pub fn stop(&self) {
        self.caddy.stop();
        self.php.stop();
        self.mysqld.stop();
    }
}

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    let mut appsvc = BlisApp::new();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    appsvc.start();

    if webbrowser::open("http://localhost:8080/").is_err() {
        println!("Couldn't open a browser");
    }

    // Run the application
    let code = app.run();

    if code != ExitCode::SUCCESS {
        println!("There's a problem boss!");
    }

    appsvc.stop();

    return code;
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
