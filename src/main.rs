#![windows_subsystem = "windows"]
/*!
    A very simple application that shows your name in a message box.
    Unlike `basic_d`, this example uses layout to position the controls in the window
*/

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;

use handlebars::Handlebars;

use std::{thread, time::Duration};

pub mod process;
use process::{php::PhpFpm, caddy::Caddy};

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 115), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,
    // #[nwg_control(text: "Heisenberg", focus: true)]
    // #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    // name_edit: nwg::TextInput,

    // #[nwg_control(text: "Say my name")]
    // #[nwg_layout_item(layout: grid, col: 0, row: 1, row_span: 2)]
    // #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    // hello_button: nwg::Button
}

impl BasicApp {
    fn say_hello(&self) {
        // nwg::modal_info_message(&self.window, "Hello", &format!("Hello {}", self.name_edit.text()));
    }

    fn say_goodbye(&self) {
        nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", "Moon"));
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("php.ini", "config/php.ini.hbs")
        .expect("Could not load php.ini.hbs");

    handlebars.set_strict_mode(true);

    let mut php = PhpFpm::spawn(handlebars);
    let mut caddy = Caddy::spawn();

    // let stdout = child_stream_to_vec(php.proc.stdout.take().expect("!stdout"));
    // println!("{}", stdout);

    thread::sleep(Duration::from_millis(5000));

    caddy.stop();
    php.stop();

    // nwg::init().expect("Failed to init Native Windows GUI");
    // nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    // let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    // nwg::dispatch_thread_events();
}
