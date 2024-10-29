use std::env;
use std::error::Error;
use std::{panic, process};
use clap;

// TODO: Update when Lazy moved to std::cell https://github.com/rust-lang/rust/pull/121377
use once_cell::sync::{Lazy};

#[path = "lib.rs"]
mod lib;
use lib::ai::{Ollama};
use lib::bash::{bash};
use lib::config::{Config};
use lib::ssh::{SshHost};

static HOME: Lazy<String> = Lazy::new(|| { env::var("HOME").unwrap().to_string() });
static DEFAULT_CONFIG_PATH: Lazy<Box<String>> = Lazy::new(|| { Box::new(format!("{}/.rusted-config.toml", HOME.to_string())) });

pub struct Cli;
impl Cli {
    pub fn new_command(command: &'static str) -> clap::Command {
        clap::Command::new(command)
    }

    pub fn new_arg(arg: &'static str) -> clap::Arg {
        clap::Arg::new(arg)
            .index(1)
            .required(true)
            .action(clap::ArgAction::Set)
    }

    fn get_arg_from_command(matches: &clap::ArgMatches, name: &str, id: &str) -> String {
        let value = matches.subcommand_matches(name).unwrap().get_one::<String>(id);
        if value.is_some() {
            value.unwrap().to_string()
        } else {
            "".to_string()
        }
    }

    pub fn run_cli(&mut self) -> Result<(), Box<dyn Error>> {

        let cli = Self::new_command("rusted")
            .author("Sarah Bennert <sarah@xhub.com>")
            .version("0.0.0")
            .arg(
                clap::Arg::new("config")
                    .required(false)
                    .action(clap::ArgAction::Set)
                    .default_value(DEFAULT_CONFIG_PATH.as_str())
            )
            .subcommand(Self::new_command("log"))
            .subcommand(Self::new_command("fix"))
            .subcommand(Self::new_command("amend"))
            .subcommand(Self::new_command("ssh-bash").arg(Self::new_arg("cluster_name")))
            .subcommand(Self::new_command("get-config").arg(Self::new_arg("cluster_name")))
            .subcommand(Self::new_command("ai")
                .arg(
                    clap::Arg::new("model")
                        .index(1)
                        .required(false)
                        .action(clap::ArgAction::Set)
                )
                .arg(
                    clap::Arg::new("command")
                        .index(2)
                        .num_args(1..)
                        .trailing_var_arg(true)
                        .required(false)
                        .action(clap::ArgAction::Set)
                )
            );

        let matches = cli.get_matches().clone();
        let subcommand = matches.subcommand_name();
        let config: Box<Config> = Config::new(matches.get_one::<String>("config").unwrap());

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
                        let destination_path = format!("{}/.kube/{}/", HOME.as_str(), cluster_name);
                        bash(format!("mkdir -p {}", destination_path).as_str(), true);
                        ssh_host.scp_from_host(source_path.as_str(), destination_path.as_str());
                        println!("Run:\nexport KUBECONFIG={}kubeconfig\n\n", destination_path)
                    }
                    _ => {}
                }
            }
            Some("ai") => {
                let model = Self::get_arg_from_command(&matches, subcommand.unwrap(), "model");
                let model_command = Self::get_arg_from_command(&matches, subcommand.unwrap(), "command");
                let local_config = config.local.unwrap();
                let config_interface = Config::get_string_from_hashmap(&local_config, "ai_interface");
                let config_model = Config::get_string_from_hashmap(&local_config, "ai_model");

                match Some(config_interface.as_str()) {
                    Some("ollama") => {
                        let ai_interface = Ollama::new();
                        match Some(model.as_str()) {
                            Some("") => {
                                ai_interface.run(format!("run {}", config_model).as_str())
                            }
                            Some("list") => {
                                ai_interface.run("list")
                            }
                            _ => {
                                ai_interface.run(format!("{} {}", model, model_command).as_str())
                            }
                        }
                    }
                    _ => {
                        panic!("Interface not implemented")
                    }
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
        let arg = Cli::new_arg("cluster_name");
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
