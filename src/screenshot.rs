use chrono::prelude::*;
use std::io;
use tokio::process::Command;
use walkdir::DirEntry;

#[derive(Debug)]
pub struct SSData {
    pub id: Option<i64>,
    pub file: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

pub async fn process_screenshot(entry: DirEntry) -> Result<SSData, io::Error> {
    let path = entry.path().to_path_buf();

    let output = Command::new("ocrs").arg(&path).output().await?;

    if output.status.success() {
        let text_in_ss = String::from_utf8_lossy(&output.stdout);
        Ok(SSData {
            id: None,
            file: path.display().to_string(),
            content: text_in_ss.to_string(),
            created_at: None,
        })
    } else {
        let err_text = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, err_text.to_string()))
    }
}
