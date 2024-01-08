extern crate create_process_w as CreateProcessW;

use std::path::PathBuf;
use CreateProcessW::Command;
use super::util::Util;

pub struct Caddy {
    proc: Option<CreateProcessW::Child>,
    exe_path: String,
    cfg_path: String,
    host: String,
    port: i32,
}

impl Caddy {
    fn caddydir() -> Result<PathBuf, &'static str> {
        let curdir = Util::root_directory()?;
        let caddy = curdir.join("bin/caddy");
        return Ok(caddy);
    }

    pub fn new() -> Caddy {
        let curdir = Util::root_directory().unwrap();
        let caddy_dir = Caddy::caddydir().unwrap();

        let exe = caddy_dir.join("caddy_windows_amd64.exe");
        let sexe = exe.to_str().unwrap();

        let cfg = curdir.join("config").join("Caddyfile");
        let scfg = cfg.to_str().unwrap();

        let _log_dir = curdir.join("log");
        let _slog_dir = _log_dir.to_str().unwrap();

        Caddy {
            proc: None,
            exe_path: String::from(sexe),
            cfg_path: String::from(scfg),
            // TODO: make these configurable
            host: String::from(""),
            port: 0,
        }
    }

    pub fn spawn(&mut self) {
        if self.proc.is_some() {
            return;
        }

        let proc = Command::new(format!("{} run --config \"{}\" ", self.exe_path.as_str(), self.cfg_path.as_str()))
            .spawn()
            .expect("Failed to launch Caddy.");

        self.proc = Some(proc);
    }

    pub fn stop(&self) {
        if self.proc.is_none() {
            return;
        }

        let proc = self.proc.as_ref().unwrap();

        proc.kill().expect("Caddy could not be killed.");
        let status = proc.wait().expect("waiting for Caddy to die failed");
        if status.success() {
            println!("Caddy was stopped.");
        } else {
            println!("Caddy could not be stopped. Status code: {}", status.code());
        }
    }
}
