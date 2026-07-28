use std::io;
use std::process::Stdio;
use tokio::process::Command;

async fn run_python(script: &str, args: &[&str]) -> Result<String, io::Error> {
    let output = Command::new("uv")
        .args(
            ["run", "--project", "agents", "python", script]
                .iter()
                .chain(args),
        )
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, err.to_string()))
    }
}

pub async fn query(query: &str, k: usize) -> Result<String, io::Error> {
    run_python("agents/query.py", &["--query", query, "--k", &k.to_string()]).await
}

pub async fn agent(query: &str) -> Result<String, io::Error> {
    run_python("agents/agent.py", &["--query", query]).await
}

pub async fn categorize(file_path: &str, text: &str) -> Result<String, io::Error> {
    run_python("agents/categorize.py", &["--file", file_path, "--text", text]).await
}
