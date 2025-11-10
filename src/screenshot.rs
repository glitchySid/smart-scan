use chrono::prelude::*;
use std::io;
use tokio::process::Command; // <-- Use tokio's Command
use walkdir::DirEntry; // <-- For error handling

#[derive(Debug)] // <-- Added Debug for easier printing
pub struct SSData {
    pub id: Option<i64>,
    pub file: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

/// Processes a single screenshot file by running 'ocrs'
/// This function is async and returns a Result
pub async fn process_screenshot(entry: DirEntry) -> Result<SSData, io::Error> {
    let path = entry.path().to_path_buf();

    // Use tokio::process::Command and .await the output
    let output = Command::new("ocrs").arg(&path).output().await?; // <-- Use .await and '?' for error handling

    if output.status.success() {
        let text_in_ss = String::from_utf8_lossy(&output.stdout);
        Ok(SSData {
            id: None,
            file: path.display().to_string(),
            content: text_in_ss.to_string(),
            created_at: None,
        })
    } else {
        // If 'ocrs' itself fails, create an error
        let err_text = String::from_utf8_lossy(&output.stderr);
        eprintln!("'ocrs' command failed for {:?}: {}", path, err_text);
        Err(io::Error::new(io::ErrorKind::Other, err_text.to_string()))
    }
}
