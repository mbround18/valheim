use clap::Parser;
use dotenv::dotenv;
use log::debug;

use commands::configure::Configuration;

use crate::commands::configure::Modifiers;
use crate::executable::handle_exit_status;
use crate::logger::debug_mode;
use crate::messages::about;
use crate::utils::is_root::is_root;

mod cli;
pub mod commands;
mod constants;
mod errors;
mod executable;
mod files;
mod logger;
mod messages;
mod mods;
mod notifications;
pub mod server;
mod steamcmd;
pub mod traits;
pub mod utils;

fn main() {
  if is_root() {
    panic!("You must run this executable without root permissions");
  }

  dotenv().ok();

  use cli::{Cli, Commands};
  let cli = Cli::parse();

  logger::initialize_logger(cli.debug || debug_mode()).unwrap();

  if cli.debug {
    debug!("Debug mode enabled!");
  }

  match cli.commands {
    Commands::Configure {
      name,
      public,
      password,
      server_executable,
      world,
      port,
      modifiers,
      preset,
      set_key,
      save_interval,
    } => Configuration::new(
      name,
      server_executable,
      port,
      world,
      password,
      { public.eq("1") }.to_owned(),
      preset,
      {
        modifiers.map(|modifiers| {
          modifiers
            .split(',')
            .map(|modifier| Modifiers::from(modifier.to_string()))
            .collect()
        })
      },
      set_key,
      save_interval,
    )
    .invoke(),
    Commands::Install {} => handle_exit_status(
      commands::install::invoke(constants::GAME_ID),
      "Successfully installed Valheim!".to_string(),
    ),
    Commands::Start {} => commands::start::invoke(cli.dry_run),
    Commands::Stop {} => commands::stop::invoke(cli.dry_run),
    Commands::Backup {
      input_directory,
      output_file,
    } => commands::backup::invoke(input_directory, output_file),
    Commands::Update { check, force } => commands::update::invoke(cli.dry_run, check, force),
    Commands::Notify {
      title,
      message,
      webhook_url,
    } => commands::notify::invoke(title, message, webhook_url),
    Commands::ModInstall { url } => commands::install_mod::invoke(url),
    Commands::Status {
      json,
      local,
      address,
    } => commands::status::invoke(json, local, address),
    Commands::About {} => {
      about(env!("GIT_HASH"));
    }
  }
}

#[cfg(test)]
mod tests {
  // use super::*;

  use clap::CommandFactory;

  use crate::cli::Cli;

  #[test]
  fn asserts() {
    Cli::command().debug_assert();
  }
}
