use std::io;
use std::io::{BufRead, Write};
use std::process::{Command, Stdio};

pub fn bash(command: &str, verify_command: bool) -> () {

    if verify_command {
        let stdin = io::stdin();
        let mut stdin_lock = stdin.lock();
        let mut input = String::new();
        let mut allow_execution: bool = false;

        let options_prompt: &[u8] = b"Please type [yes] to continue, [skip] to skip this step, [exit] to exit: \n";

        println!("Execute? {:?}", command);
        while !allow_execution {
            let _ = io::stderr().write(options_prompt);
            while let Ok(_len) = stdin_lock.read_line(&mut input) {
                input = input.strip_suffix("\n").unwrap().to_string();
                if input.eq(&"yes") {
                    allow_execution = true;
                    break;
                } else if input.eq(&"skip") {
                    return;
                } else if input.eq(&"exit") {
                    panic!("exit!");
                } else {
                    input.clear();
                    let _ = io::stderr().write(options_prompt);
                    continue;
                }
            }
            if allow_execution {
                break;
            }
        }
    }

    let mut child = Command::new("/usr/bin/env")
        .arg("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Command failed");
    let status = child.wait();
    println!("bash exit: {:0}", status.unwrap().code().unwrap())
}
