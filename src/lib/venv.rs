#[path = "bash.rs"]
mod bash;

use std::path::PathBuf;
use bash::{bash};


static PYTHON:&str = "python3";
static VENV_DIR:&str = ".venv";

pub struct Venv {
    pub directory: String,
    pub activate: String,
}
impl Venv {
    pub fn new(path: &str) -> Self {
        // Calling function must create the directory
        let venv_path:PathBuf = [PathBuf::from(path), PathBuf::from(VENV_DIR)].iter().collect();
        let activate:PathBuf = [&venv_path, &PathBuf::from("bin/activate")].iter().collect();

        let bash_cmd = format!("cd {path:?} && \
            {PYTHON:?} -m venv --upgrade-deps {venv_path:?} && \
            source {activate:?}");
        bash(bash_cmd.as_str(), false);


        Self{
            directory: path.to_string(),
            activate: activate.to_str().unwrap().to_string(),
        }
    }
    pub fn bash(&self, command: &str, verify_command: bool) -> () {
        let bash_cmd = format!(
            "cd {directory:?} && source {activate:?} && bash -c {command:?}",
            directory=&self.directory,
            activate=&self.activate
        );
        bash(bash_cmd.as_str(), verify_command)
    }
}
