#[tauri::command]
pub fn get_platform() -> String {
  #[cfg(target_os = "windows")]
  return "windows".to_string();
  #[cfg(target_os = "macos")]
  return "macos".to_string();
  #[cfg(target_os = "linux")]
  return "linux".to_string();
  #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
  return "unknown".to_string();
}