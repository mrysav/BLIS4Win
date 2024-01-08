#![windows_subsystem = "windows"]
/*!
    A very simple application that shows your name in a message box.
    Unlike `basic_d`, this example uses layout to position the controls in the window
*/

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwg::NativeUi;

use handlebars::Handlebars;

pub mod process;
use process::{
    caddy::{self},
    php::{self},
};

const BLIS_VERSION: &'static str = "4.0";

pub struct BlisApp {
    window: nwg::Window,
    layout: nwg::GridLayout,

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
            window: Default::default(),
            layout: Default::default(),
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

        nwg::stop_thread_dispatch();
    }
}

use std::{cell::RefCell, borrow::Borrow, thread::sleep, time::Duration};
use std::ops::Deref;
use std::rc::Rc;

pub struct BlisAppUi {
    inner: Rc<BlisApp>,
    default_handler: RefCell<Vec<nwg::EventHandler>>,
}

impl nwg::NativeUi<BlisAppUi> for BlisApp {
    fn build_ui(mut data: BlisApp) -> Result<BlisAppUi, nwg::NwgError> {
        use nwg::Event as E;

        // Controls
        nwg::Window::builder()
            .size((300, 150))
            .position((300, 300))
            .title(format!("BLIS v{}", BLIS_VERSION).as_str())
            .build(&mut data.window)?;

        // Wrap-up
        let ui = BlisAppUi {
            inner: Rc::new(data),
            default_handler: Default::default(),
        };

        // Events
        let window_handles = [&ui.window.handle];
        for handle in window_handles.iter() {
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(evt_ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnWindowClose => {
                            if &handle == &evt_ui.window {
                                BlisApp::stop(&evt_ui);
                            }
                        }
                        _ => {}
                    }
                }
            };

            ui.default_handler
                .borrow_mut()
                .push(nwg::full_bind_event_handler(handle, handle_events));
        }

        // Layouts
        nwg::GridLayout::builder()
            .parent(&ui.window)
            .spacing(2)
            .min_size([150, 140])
            .build(&ui.layout)?;

        return Ok(ui);
    }
}

impl Drop for BlisAppUi {
    /// To make sure that everything is freed without issues, the default handler must be unbound.
    fn drop(&mut self) {
        let mut handlers = self.default_handler.borrow_mut();
        for handler in handlers.drain(0..) {
            nwg::unbind_event_handler(&handler);
        }
    }
}

impl Deref for BlisAppUi {
    type Target = BlisApp;

    fn deref(&self) -> &BlisApp {
        return &self.inner;
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let mut app = BlisApp::new();
    app.start();
    let _app = BlisApp::build_ui(app).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
