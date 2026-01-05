#[cfg(target_os = "windows")]
pub fn detect_wallpaper_path() -> Option<String> {
    use std::path::PathBuf;
    use windows::Win32::UI::WindowsAndMessaging::{
        SPI_GETDESKWALLPAPER, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, SystemParametersInfoW,
    };

    let mut buffer = [0u16; 260];
    let result = unsafe {
        SystemParametersInfoW(
            SPI_GETDESKWALLPAPER,
            buffer.len() as u32,
            Some(buffer.as_mut_ptr() as *mut _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
    };

    if result.is_ok() {
        let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
        let path = String::from_utf16_lossy(&buffer[..len]);
        if !path.is_empty() && PathBuf::from(&path).exists() {
            return Some(path);
        }
    }
    None
}

#[cfg(target_os = "macos")]
pub fn detect_wallpaper_path() -> Option<String> {
    use std::path::PathBuf;
    use std::process::Command;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(r#"tell application "System Events" to get picture of desktop 1"#)
        .output()
        .ok()?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !path.is_empty() && PathBuf::from(&path).exists() {
            return Some(path);
        }
    }
    None
}

#[cfg(target_os = "linux")]
pub fn detect_wallpaper_path() -> Option<String> {
    None
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn detect_wallpaper_path() -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_wallpaper() {
        let path = detect_wallpaper_path();
        println!("Detected wallpaper path: {:?}", path);
    }
}
