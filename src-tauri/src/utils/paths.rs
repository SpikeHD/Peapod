use std::{fs, path::PathBuf};

use crate::utils::config::{default_config, get_config};
use crate::log;

pub fn is_portable() -> bool {
  let current_exe = std::env::current_exe().unwrap_or_default();
  let portable_signifier = current_exe.parent().unwrap().join(".portable");

  fs::metadata(portable_signifier).is_ok()
}

pub fn get_config_dir() -> PathBuf {
  // First check for a local config file
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_config_dir = current_exe.parent().unwrap().join("config.json");

  if is_portable() {
    // Create file if it doesn't exist
    if fs::metadata(&local_config_dir).is_err() {
      fs::write(
        &local_config_dir,
        serde_json::to_string_pretty(&default_config()).unwrap_or_default(),
      )
      .unwrap_or(());
    }

    return local_config_dir;
  }

  log!("No local config file found. Using default.");

  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  let config_file = appdata.join("peapod").join("config.json");

  if fs::metadata(appdata.join("peapod")).is_err() {
    fs::create_dir_all(appdata.join("peapod")).expect("Error creating appdata dir");
  }

  // Write default config if it doesn't exist
  if fs::metadata(&config_file).is_err() {
    fs::write(
      &config_file,
      serde_json::to_string_pretty(&default_config()).unwrap_or_default(),
    )
    .unwrap_or(());
  }

  config_file
}

pub fn config_is_local() -> bool {
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_config_dir = current_exe.parent().unwrap().join("config.json");

  fs::metadata(local_config_dir).is_ok()
}

pub fn log_file_path() -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();

  if is_portable() {
    // This is a portable install, so we can use the local dir
    return current_exe
      .parent()
      .unwrap()
      .join("logs")
      .join("latest.log");
  }

  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  appdata.join("peapod").join("logs").join("latest.log")
}

pub fn get_webdata_dir() -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();

  if is_portable() {
    // This is a portable install, so we can use the local dir
    return current_exe
      .parent()
      .unwrap()
      .join("webdata");
  }
  
  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  appdata.join("peapod").join("webdata")
}