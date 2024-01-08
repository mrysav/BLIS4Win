use std::env::current_exe;
use std::path::PathBuf;

pub struct Util {}

impl Util {
    pub fn root_directory() -> Result<PathBuf, &'static str> {
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
}
