#[path = "bash.rs"]
mod bash;
use bash::{bash};

pub struct SshHost {
    pub host: Box<String>,
    pub user: Box<String>,
    pub key: Box<String>,
}

impl SshHost {
    pub fn bash(&self, remote_dir: &str, remote_command: &str) -> () {
        let ssh_cmd: String = format!("ssh -i {:?} {}@{}", self.key, self.user, self.host);
        let bash_cmd: String = format!("{} /usr/bin/env bash -l -c 'cd {:?} && {}'", ssh_cmd, remote_dir, remote_command);
        bash(bash_cmd.as_str(), true)
    }

    pub fn scp_from_host(&self, source_path: &str, destination_path: &str) -> () {
        let scp_cmd: String = format!("scp -i {} {}@{}:{} {}", self.key, self.user, self.host, source_path, destination_path);
        bash(scp_cmd.as_str(), true)
    }
}
