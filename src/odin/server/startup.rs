use daemonize::{Daemonize, DaemonizeError};
use log::{debug, error, info};

use std::{io, process::Child};

use crate::mods::bepinex::BepInExEnvironment;
use crate::utils::common_paths::{game_directory, saves_directory};
use crate::utils::environment::fetch_var;
use crate::{
  constants,
  executable::create_execution,
  files::{create_file, ValheimArguments},
  messages,
  utils::environment,
};
use std::process::exit;

type CommandResult = io::Result<Child>;

pub fn start_daemonized(config: ValheimArguments) -> Result<CommandResult, DaemonizeError> {
  let stdout = create_file(format!("{}/logs/valheim_server.log", game_directory()).as_str());
  let stderr = create_file(format!("{}/logs/valheim_server.err", game_directory()).as_str());
  Daemonize::new()
    .working_directory(game_directory())
    .user("steam")
    .group("steam")
    .stdout(stdout)
    .stderr(stderr)
    .exit_action(|| {
      let bepinex_env = BepInExEnvironment::new();
      if bepinex_env.is_installed() {
        info!("Server has been started with BepInEx! Keep in mind this may cause errors!!");
        messages::modding_disclaimer();
        debug!("{:#?}", bepinex_env);
      }
      info!("Server has been started and Daemonized. It should be online shortly!");
      info!("Keep an eye out for 'Game server connected' in the log!");
      info!("(this indicates its online without any errors.)")
    })
    .privileged_action(move || start(&config))
    .start()
}

pub fn start(config: &ValheimArguments) -> CommandResult {
  let mut command = create_execution(&config.command);
  info!("--------------------------------------------------------------------------------------------------------------");
  let ld_library_path_value = environment::fetch_multiple_var(
    constants::LD_LIBRARY_PATH_VAR,
    format!("{}/linux64", game_directory()).as_str(),
  );
  debug!("Setting up base command");
  let mut base_command = command
    // Extra launch arguments
    .arg(fetch_var(
      "SERVER_EXTRA_LAUNCH_ARGS",
      "-nographics -batchmode",
    ))
    // Required vars
    .args(&[
      "-port",
      &config.port.as_str(),
      "-name",
      &config.name.as_str(),
      "-world",
      &config.world.as_str(),
      "-public",
      &config.public.as_str(),
    ])
    .env("SteamAppId", environment::fetch_var("APPID", "892970"))
    .current_dir(game_directory());

  let is_public = config.public.eq("1");
  let is_vanilla = fetch_var("TYPE", "vanilla").eq_ignore_ascii_case("vanilla");
  let no_password = config.password.is_empty();

  // If no password env variable
  if !is_public && !is_vanilla && no_password {
    debug!("No password found, skipping password flag.")
  } else if no_password && (is_public || is_vanilla) {
    error!("Cannot run you server with no password! PUBLIC must be 0 and cannot be a Vanilla type server.");
    exit(1)
  } else {
    debug!("Password found, adding password flag.");
    base_command = base_command.args(&["-password", &config.password.as_str()]);
  }

  // Tack on save dir at the end.
  base_command = base_command.args(&["-savedir", &saves_directory()]);

  info!("Executable: {}", &config.command);
  info!("Launching Command...");
  let bepinex_env = BepInExEnvironment::new();
  if bepinex_env.is_installed() {
    info!("BepInEx detected! Switching to run with BepInEx...");
    debug!("BepInEx Environment: \n{:#?}", bepinex_env);
    bepinex_env.launch(base_command)
  } else {
    info!("Everything looks good! Running normally!");
    base_command
      .env(constants::LD_LIBRARY_PATH_VAR, ld_library_path_value)
      .spawn()
  }
}
