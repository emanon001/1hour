use anyhow::Result;
use std::{io::Write, process::Command, process::Output, process::Stdio};

pub struct Jq {
    path: String,
    json: String,
}

impl Jq {
    pub fn new(path: &str, json: &str) -> Self {
        Self {
            path: path.to_string(),
            json: json.to_string(),
        }
    }

    pub fn execute(&self, filter: &str) -> Result<Output> {
        let mut jq = Command::new(&self.path)
            .arg(filter)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let jq_stdin = jq.stdin.as_mut().unwrap();
        jq_stdin.write_all(self.json.as_bytes())?;

        let output = jq.wait_with_output()?;
        Ok(output)
    }
}
