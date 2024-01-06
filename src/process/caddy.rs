extern crate create_process_w as CreateProcessW;

use std::{env::current_exe, path::PathBuf};
use CreateProcessW::Command;

pub struct Caddy {
    pub proc: CreateProcessW::Child,
}

impl Caddy {
    fn curdir() -> Result<PathBuf, &'static str> {
        let mut dir = current_exe().unwrap();

        while dir.pop() {
            let cfg_pb = dir.join("config");
            if cfg_pb.as_path().exists() {
                // config dir exists, this is the path we want
                return Ok(dir);
            }
        }

        return Err("Could not find 'config' directory.");
    }

    fn caddydir() -> Result<PathBuf, &'static str> {
        let curdir = Caddy::curdir()?;
        let caddy = curdir.join("bin/caddy");
        return Ok(caddy);
    }

    pub fn spawn() -> Caddy {
        let curdir = Caddy::curdir().unwrap();
        let caddy_dir = Caddy::caddydir().unwrap();

        let exe = caddy_dir.join("caddy_windows_amd64.exe");
        let sexe = exe.to_str().unwrap();

        let cfg = curdir.join("config").join("Caddyfile");
        let scfg = cfg.to_str().unwrap();

        let _log_dir = curdir.join("log");
        let _slog_dir = _log_dir.to_str().unwrap();

        let proc = Command::new(format!("{} run --config {}", sexe, scfg))
            .spawn()
            .expect("Failed to launch Caddy.");

        Caddy { proc: proc }
    }

    pub fn stop(&mut self) {
        self.proc.kill().expect("Caddy could not be killed.");
        let status = self.proc.wait().expect("waiting for Caddy to die failed");
        if status.success() {
            println!("Caddy was stopped.");
        } else {
            println!("Caddy could not be stopped. Status code: {}", status.code());
        }
    }
}
