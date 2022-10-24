use std::io::Write;
use std::process::{Command, ExitStatus, Stdio};

use anyhow::{Context, Result};

const MAN: &str = include_str!("../target/assets/doc/man/souffle-lint.1");

pub fn man() -> Result<ExitStatus> {
    let mut child = Command::new("man")
        .args(["--local-file", "-"])
        .stdin(Stdio::piped())
        .spawn()
        .context("Failed to launch `man`")?;
    child
        .stdin
        .as_ref()
        .map(|mut sin| sin.write(MAN.as_bytes()));
    Ok(child.wait()?)
}
