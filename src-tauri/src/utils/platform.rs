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

#[cfg(target_os = "windows")]
pub fn disable_hardware_accel_windows() {
  let existing_args = std::env::var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS").unwrap_or_default();
  std::env::set_var(
    "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
    format!("{} --disable-gpu", existing_args),
  );
}

#[cfg(target_os = "linux")]
pub fn disable_hardware_accel_linux(window: &tauri::WebviewWindow) {
  use crate::log;
  use webkit2gtk::{HardwareAccelerationPolicy, SettingsExt, WebViewExt};

  window.with_webview(move |webview| {
    let wv = webview.inner();
    let settings = WebViewExt::settings(&wv).unwrap_or_default();

    settings.set_hardware_acceleration_policy(HardwareAccelerationPolicy::Never);
  }).unwrap_or_else(|_| log!("Failed to set user-agent"));
}