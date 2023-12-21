extern crate create_process_w as CreateProcessW;

use CreateProcessW::Command;
use std::process::Stdio;

pub struct PhpFpm {
    pub proc: CreateProcessW::Child,
}

impl PhpFpm {
    pub fn spawn() -> PhpFpm {
        let exe = String::from("php-5.6.9-Win32-VC11-x64/php-cgi.exe");

        let host = String::from("127.0.0.1");
        let port = 8112;
        let bindto = format!("{}:{}", host, port);

        let proc = Command::new(format!("{} -b {}", exe, bindto))
            .spawn()
            .expect("Failed to launch PHP.");

        PhpFpm {
            proc: proc,
        }
    }

    pub fn stop(&mut self) {
        self.proc.kill().expect("PHP could not be killed.")
    }
}
