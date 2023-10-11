use log::debug;
use serde::{Deserialize, Serialize};

use crate::files::config::{config_file, write_config};
use crate::files::discord::{discord_file, write_discord};

/// See: https://user-images.githubusercontent.com/34519392/273088066-b9c94664-9eef-419d-999a-8b8798462dee.PNG
/// for a list of modifiers
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Modifiers {
  /// The name of the modifier
  pub name: String,

  /// The value of the modifier
  pub value: String,
}

impl From<String> for Modifiers {
  /// Creates a new modifier from a string
  fn from(value: String) -> Self {
    let mut split = value.split('=');
    let name = split.next().unwrap().to_string();
    let value = split.next().unwrap().to_string();
    Modifiers { name, value }
  }
}

pub struct Configuration {
  /// Sets the name of the server, (Can be set with ENV variable NAME)
  pub name: String,

  /// Sets the servers executable path.
  pub server_executable: String,

  /// Sets the port of the server, (Can be set with ENV variable PORT)
  pub port: u16,

  /// Sets the world of the server, (Can be set with ENV variable WORLD)
  pub world: String,

  /// Sets the password of the server, (Can be set with ENV variable PASSWORD)
  pub password: String,

  /// Sets the public state of the server, (Can be set with ENV variable PUBLIC)
  pub public: bool,

  /// Sets flag preset for launching the server, (Can be set with ENV variable PRESET)
  pub preset: Option<String>,

  /// Sets flag modifiers for launching the server, (Can be set with ENV variable MODIFIERS)
  pub modifiers: Option<Vec<Modifiers>>,

  /// Sets flag set_key for launching the server, (Can be set with ENV variable SET_KEY)
  pub set_key: Option<String>,

  /// Sets the save interval in seconds
  pub save_interval: Option<u16>,
}

impl Configuration {
  /// Creates a new configuration
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    name: String,
    server_executable: String,
    port: u16,
    world: String,
    password: String,
    public: bool,
    preset: Option<String>,
    modifiers: Option<Vec<Modifiers>>,
    set_key: Option<String>,
    save_interval: Option<u16>,
  ) -> Self {
    Configuration {
      name,
      server_executable,
      port,
      world,
      password,
      public,
      preset,
      modifiers,
      set_key,
      save_interval,
    }
  }

  /// Invokes the configuration by writing the config file
  pub fn invoke(self) {
    debug!("Pulling config file...");
    let config = config_file();
    debug!("Writing config file...");
    write_config(config, self);
    debug!("Pulling Discord config file...");
    let discord = discord_file();
    debug!("Writing Discord config file...");
    write_discord(discord);
  }
}
