extern crate create_process_w as CreateProcessW;

use super::util::Util;
use core::panic;
use std::{collections::HashMap, fs::File, io::prelude::*, path::PathBuf};
use CreateProcessW::Command;

pub struct Mysqld {
    // Inner process of Mysqld, once it is spawned
    proc: Option<CreateProcessW::Child>,
    exe_path: String,
    datadir: String,
    host: String,
    port: i32,
}

impl Mysqld {
    fn mysqldir() -> Result<PathBuf, &'static str> {
        let curdir = Util::root_directory()?;
        let mysql = curdir.join("bin").join("mysql-8.0.36-winx64");
        return Ok(mysql);
    }

    pub fn new() -> Mysqld {
        let curdir = Util::root_directory().unwrap();
        let mysqld_dir = Mysqld::mysqldir().unwrap();
        let exe = mysqld_dir.join("bin\\mysqld.exe");
        let datadir = mysqld_dir.join("data");

        // unused right now...
        let host = String::from("127.0.0.1");
        let port = 8112;

        Mysqld {
            proc: None,
            exe_path: String::from(exe.to_str().unwrap()),
            datadir: String::from(datadir.to_str().unwrap()),
            // TODO: make these configurable
            host: String::from("127.0.0.1"),
            port: 8112,
        }
    }

    pub fn spawn(&mut self) {
        if self.proc.is_some() {
            return;
        }

        let proc = Command::new(format!(
            "{} --console --datadir \"{}\"",
            self.exe_path, self.datadir
        ))
        .spawn()
        .expect("Failed to launch Mysqld.");

        self.proc = Some(proc);
    }

    pub fn stop(&self) {
        if self.proc.is_none() {
            return;
        }

        let proc = self.proc.as_ref().unwrap();

        match proc.kill() {
            Ok(_p) => {}
            Err(error) => {
                println!("Failed killing mysqld; might already be dead");
                return;
            }
        }

        let status = proc.wait().expect("waiting for mysqld to die failed");
        if status.success() {
            println!("Mysqld was stopped.");
        } else {
            println!(
                "Mysqld could not be stopped. Status code: {}",
                status.code()
            );
        }
    }
}
