use std::error::Error;
use std::{panic, process};
use clap;

#[path = "lib.rs"]
mod lib;
use lib::bash::{bash};
use lib::config::{Config};
use lib::ssh::{SshHost};


pub struct Cli;
impl Cli {
    pub fn new_command(command: &'static str) -> clap::Command {
        clap::Command::new(command)
    }

    pub fn new_cluster_name_arg() -> clap::Arg {
        clap::Arg::new("cluster_name")
            .index(1)
            .required(true)
            .action(clap::ArgAction::Set)
    }

    fn get_arg_from_command(matches: &clap::ArgMatches, name: &str, id: &str) -> String {
        matches.subcommand_matches(name).unwrap().get_one::<String>(id).unwrap().to_string()
    }

    pub fn run_cli(&mut self) -> Result<(), Box<dyn Error>> {
        let cli = Self::new_command("rusted")
            .author("Sarah Bennert <sarah@xhub.com>")
            .version("0.0.0")
            .subcommand(Self::new_command("log"))
            .subcommand(Self::new_command("fix"))
            .subcommand(Self::new_command("amend"))
            .subcommand(Self::new_command("ssh-bash").arg(Self::new_cluster_name_arg()))
            .subcommand(Self::new_command("get-config").arg(Self::new_cluster_name_arg()));

        let matches = cli.get_matches().clone();
        let subcommand = matches.subcommand_name();
        let config: Box<Config> = Config::new("config.toml");

        match subcommand {
            Some("log") => {
                bash("git log -n 5 --oneline", false);
            }
            Some("fix") => {
                bash("git rebase -i HEAD~2", false);
            }
            Some("amend") => {
                bash("git commit --amend", false);
            }
            Some("ssh-bash") | Some("get-config") => {
                let cluster_name = Self::get_arg_from_command(&matches, subcommand.unwrap(), "cluster_name").into_boxed_str();
                let host_config = config.get_host_config(&cluster_name);
                let auth_config = config.get_auth_config(&host_config);
                let ssh_host = SshHost {
                    host: config.get_hostname(&host_config, &auth_config, &cluster_name),
                    user: config.get_auth(&auth_config, "user"),
                    key: config.get_ssh_key(&auth_config)
                };
                match subcommand {
                    Some("ssh-bash") => {
                        ssh_host.bash("~", "/usr/bin/env bash -i");
                    }
                    Some("get-config") => {
                        let cluster_directory = config.get_remote_cluster_directory(&host_config);
                        let source_path = format!("\"{}/{}/auth/*\"", cluster_directory, cluster_name);
                        let destination_path = format!("~/.kube/{}/", cluster_name);
                        bash(format!("mkdir -p ~/.kube/{}", cluster_name).as_str(), true);
                        ssh_host.scp_from_host(source_path.as_str(), destination_path.as_str());
                        println!("Run:\nexport KUBECONFIG=~/.kube/{}/kubeconfig\n\n", cluster_name)
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::Cli;

    #[test]
    fn test_new_command() -> io::Result<()> {
        let cmd_str = "test_command";
        let cmd = Cli::new_command(cmd_str);
        assert_eq!(cmd_str, cmd.get_name());
        Ok(())
    }

    #[test]
    fn test_new_cluster_name_arg() -> io::Result<()> {
        let arg = Cli::new_cluster_name_arg();
        assert_eq!("cluster_name", arg.get_id());
        Ok(())
    }

}


fn main() -> Result<(), Box<dyn Error>> {
    // Signal handling is a known issue in Rust
    // https://github.com/rust-lang/rfcs/issues/1368

    let mut rusted = Cli{};

    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        println!("Custom panic hook");
        orig_hook(panic_info);
        process::exit(1);
    }));


    let result = rusted.run_cli();
    match result {
        Ok(description) => {
            eprintln!("{:?}", description);
            Ok(description)
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(err)
        },
    }
}
