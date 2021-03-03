pub mod environment;

use clap::ArgMatches;
use log::debug;
use std::env;
use std::path::Path;

const ODIN_WORKING_DIR: &str = "ODIN_WORKING_DIR";
pub const VALHEIM_EXECUTABLE_NAME: &str = "valheim_server.x86_64";

pub fn get_working_dir() -> String {
  environment::fetch_var(
    ODIN_WORKING_DIR,
    env::current_dir().unwrap().to_str().unwrap(),
  )
}

pub fn get_variable(args: &ArgMatches, name: &str, default: String) -> String {
  debug!("Checking env for {}", name);
  if let Ok(env_val) = env::var(name.to_uppercase()) {
    if !env_val.is_empty() {
      debug!("Env variable found {}={}", name, env_val);
      return env_val;
    }
  }
  if let Ok(env_val) = env::var(format!("SERVER_{}", name).to_uppercase()) {
    debug!("Env variable found {}={}", name, env_val);
    return env_val;
  }
  args
    .value_of(name)
    .unwrap_or_else(|| default.as_str())
    .to_string()
}

pub fn server_installed() -> bool {
  Path::new(&[get_working_dir(), VALHEIM_EXECUTABLE_NAME.to_string()].join("/")).exists()
}

pub(crate) fn path_exists(path: &str) -> bool {
  let state = Path::new(path).exists();
  debug!(
    "Path {} {}",
    path,
    if state { "exists" } else { "does not exist" }
  );
  state
}
