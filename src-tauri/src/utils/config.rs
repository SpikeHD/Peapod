use serde::{Deserialize, Serialize};
use std::fs;

use crate::utils::paths::get_config_dir;
use crate::log;

#[derive(Serialize, Deserialize)]
pub struct Config {}

pub fn init() {
  get_config_dir();
}

#[tauri::command]
pub fn read_config_file() -> String {
  init();

  let config_file = get_config_dir();

  fs::read_to_string(config_file).expect("Config does not exist!")
}

#[tauri::command]
pub fn write_config_file(contents: String) {
  init();

  let config_file = get_config_dir();

  fs::write(config_file, contents).expect("Error writing config!")
}

#[tauri::command]
pub fn default_config() -> Config {
  Config {}
}

#[tauri::command]
pub fn get_config() -> Config {
  let config_str = read_config_file();
  let config_str = config_str.as_str();

  match serde_json::from_str(config_str) {
    Ok(config) => config,
    Err(e) => {
      log!("Failed to parse config, using default config!");
      log!("Error: {}", e);

      default_config()
    }
  }
}

#[tauri::command]
pub fn set_config(config: Config) {
  let config_str = serde_json::to_string(&config).unwrap();
  write_config_file(config_str);
}
