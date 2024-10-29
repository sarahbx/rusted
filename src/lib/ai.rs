use std::env;
use std::fs;
use std::path::PathBuf;

#[path = "rand.rs"]
mod rand;
use rand::rand_alphanum_string;

#[path = "venv.rs"]
mod venv;
use venv::Venv;

pub struct Ollama {
    venv: Venv,
}
impl Ollama {
    pub fn new() -> Self {
        let random_string:String = rand_alphanum_string(16);
        let new_temp_dir:PathBuf = [env::temp_dir(), PathBuf::from(random_string)].iter().collect();
        let _create = fs::create_dir(&new_temp_dir);
        Self{ venv: Venv::new(new_temp_dir.to_str().unwrap()) }
    }

    pub fn run(&self, ollama_command: &str) {
        for _command in [
            "ollama serve > ~/.ollama-serve.log 2>&1 &".to_string(),
            format!("sleep 1; ollama {ollama_command}"),
        ] {
            self.venv.bash(_command.as_str(), false)
        }
    }
}
impl Drop for Ollama {
    fn drop(&mut self) {
        self.venv.bash("killall -9 ollama; killall -9 ollama_llama_server", false);
        let _remove = fs::remove_dir(&self.venv.directory);
    }
}
