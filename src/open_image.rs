#[cfg(target_os = "macos")]
use tokio::process::Command;

#[cfg(target_os = "macos")]
pub fn open(path: &str) {
    #[cfg(target_os = "windows")]
    {
        // On Windows, use cmd.exe /C start
        let _ = Command::new("cmd.exe")
            .arg("/C")
            .arg("start")
            .arg("")
            .arg(path)
            .spawn(); // Use .spawn() for "fire and forget"
    }

    #[cfg(target_os = "macos")]
    {
        // On macOS, use 'open'
        let _ = Command::new("open").arg(path).spawn();
    }

    #[cfg(target_os = "linux")]
    {
        // On Linux, use 'xdg-open'
        let _ = Command::new("xdg-open").arg(path).spawn();
    }
}
