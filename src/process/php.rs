extern crate create_process_w as CreateProcessW;

use super::util::Util;
use core::panic;
use handlebars::Handlebars;
use std::{collections::HashMap, fs::File, io::prelude::*, path::PathBuf};
use CreateProcessW::Command;

pub struct PhpFpm {
    // Inner process of PHP, once it is spawned
    proc: Option<CreateProcessW::Child>,
    // The complete, rendered version of php.ini
    php_ini: String,
    exe_path: String,
    cfg_path: String,
    host: String,
    port: i32,
}

impl PhpFpm {
    fn phpdir() -> Result<PathBuf, &'static str> {
        let curdir = Util::root_directory()?;
        let php = curdir.join("bin").join("php-5.6.9-Win32-VC11-x64");
        return Ok(php);
    }

    pub fn new(handlebars: Handlebars) -> PhpFpm {
        let curdir = Util::root_directory().unwrap();
        let php_dir = PhpFpm::phpdir().unwrap();
        let exe = php_dir.join("php-cgi.exe");
        let cfg = curdir.join("run").join("php.ini");

        let log_dir = curdir.join("log").into_os_string();

        let mut vars = HashMap::new();

        let slog_dir = log_dir
            .into_string()
            .expect("could not make log_dir a string");
        vars.insert("log_dir", slog_dir);

        let ophp_dir = php_dir.into_os_string();
        let sphp_dir = ophp_dir
            .into_string()
            .expect("couldn't make php_dir a string");
        vars.insert("php_exe_dir", sphp_dir);

        let rendered_cfg = handlebars.render("php.ini", &vars).unwrap();

        let host = String::from("127.0.0.1");
        let port = 8112;

        PhpFpm {
            proc: None,
            php_ini: rendered_cfg,
            exe_path: String::from(exe.to_str().unwrap()),
            cfg_path: String::from(cfg.to_str().unwrap()),
            // TODO: make these configurable
            host: String::from("127.0.0.1"),
            port: 8112,
        }
    }

    pub fn spawn(&mut self) {
        if self.proc.is_some() {
            return;
        }

        let mut file = match File::create(self.cfg_path.as_str()) {
            Err(why) => panic!("couldn't create {}: {}", self.cfg_path, why),
            Ok(file) => file,
        };

        match file.write_all(self.php_ini.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", self.cfg_path, why),
            Ok(_) => println!("successfully wrote to {}", self.cfg_path),
        }

        let proc = Command::new(format!(
            "{} -b {}:{} -c {}",
            self.exe_path, self.host, self.port, self.cfg_path
        ))
        .spawn()
        .expect("Failed to launch PHP.");

        self.proc = Some(proc);
    }

    pub fn stop(&self) {
        if self.proc.is_none() {
            return;
        }

        let proc = self.proc.as_ref().unwrap();
        proc.kill().expect("PHP could not be killed.");
        let status = proc.wait().expect("waiting for PHP to die failed");
        if status.success() {
            println!("PHP was stopped.");
        } else {
            println!("PHP could not be stopped. Status code: {}", status.code());
        }
    }
}
