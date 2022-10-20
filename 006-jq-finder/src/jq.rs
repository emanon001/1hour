use anyhow::Result;
use std::{io::Write, process::Command, process::Output, process::Stdio};

pub fn execute_jq(jq_path: &str, json: &str, filter: &str) -> Result<Output> {
    let mut jq = Command::new(jq_path)
        .arg(filter)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let jq_stdin = jq.stdin.as_mut().unwrap();
    jq_stdin.write_all(json.as_bytes())?;

    let output = jq.wait_with_output()?;
    Ok(output)
}
