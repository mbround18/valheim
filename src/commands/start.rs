use crate::executable::{create_execution};
use std::process::{Stdio};
use clap::{ArgMatches};
use crate::utils::{get_variable, server_installed, get_working_dir};
use log::{info, error};
use crate::files::start_server_rusty::{write_rusty_start_script, ValheimArguments};
use crate::files::server_exit;

pub fn invoke(args: &ArgMatches) {
    server_exit::delete_if_exist();
    info!("Setting up start scripts...");
    let mut command = create_execution("bash");
    let server_executable = &[get_working_dir(),  "valheim_server.x86_64".to_string()].join("/");
    let script_args = &ValheimArguments {
        port: get_variable(args, "port", "2456".to_string()).to_string(),
        name: get_variable(args, "name", "Valheim powered by Odin".to_string()),
        world: get_variable(args, "world", "Dedicated".to_string()),
        password: get_variable(args, "password", "12345".to_string()),
        command: server_executable.to_string()
    };
    let dry_run: bool = args.is_present("dry_run");
    info!("Looking for burial mounds...");
    write_rusty_start_script(script_args, dry_run);
    if !dry_run {
        if server_installed() {
            let updated_command = command
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .arg("-c")
                .arg("./start_server_rusty.sh")
                .env("LD_LIBRARY_PATH", "${PWD}/linux64:${LD_LIBRARY_PATH}");
            match updated_command.output() {
                Ok(output) => {
                    info!("Exit with code {}", output.status);
                    info!("Server has started...");
                    info!("Check out ./output.log for the logs.");
                    info!("Keep an eye out for \"Game server connected\" and you server should be live!");
                },
                _ => {
                    error!("An error has occurred!")
                }
            }
        } else {
            error!("Could not find server executable! Please install the server!")
        }
    }
}
