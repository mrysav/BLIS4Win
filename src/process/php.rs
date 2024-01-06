extern crate create_process_w as CreateProcessW;

use core::panic;
use handlebars::Handlebars;
use std::{collections::HashMap, env::current_exe, fs::File, io::prelude::*, path::PathBuf};
use CreateProcessW::Command;

pub struct PhpFpm {
    pub proc: CreateProcessW::Child,
}

impl PhpFpm {
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

    fn phpdir() -> Result<PathBuf, &'static str> {
        let curdir = PhpFpm::curdir()?;
        let php = curdir.join("bin/php-5.6.9-Win32-VC11-x64");
        return Ok(php);
    }

    pub fn spawn(handlebars: Handlebars) -> PhpFpm {
        let curdir = PhpFpm::curdir().unwrap();
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

        let mut file = match File::create(&cfg) {
            Err(why) => panic!("couldn't create {}: {}", cfg.display(), why),
            Ok(file) => file,
        };

        match file.write_all(rendered_cfg.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", cfg.display(), why),
            Ok(_) => println!("successfully wrote to {}", cfg.display()),
        }

        let host = String::from("127.0.0.1");
        let port = 8112;
        let bindto = format!("{}:{}", host, port);

        let proc = Command::new(format!(
            "{} -b {} -c {}",
            exe.display(),
            bindto,
            cfg.display()
        ))
        .spawn()
        .expect("Failed to launch PHP.");

        PhpFpm { proc: proc }
    }

    pub fn stop(&mut self) {
        self.proc.kill().expect("PHP could not be killed.");
        let status = self.proc.wait().expect("waiting for PHP to die failed");
        if status.success() {
            println!("PHP was stopped.");
        } else {
            println!("PHP could not be stopped. Status code: {}", status.code());
        }
    }
}
