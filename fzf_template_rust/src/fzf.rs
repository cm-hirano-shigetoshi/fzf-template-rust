use std::env;
use std::io::{Error, Result};
use std::process::{Command, Stdio};

pub struct Fzf {
    pub proc: Option<std::process::Child>,
}

impl Fzf {
    pub fn start(&mut self) -> Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(
                format!("fd | fzf --listen {} --bind 'alt-j:execute-silent:curl \"http://localhost:{}?bind=alt-j\"'", env::var("FZF_PORT").unwrap(), env::var("SERVER_PORT").unwrap())
            )
            .stderr(Stdio::inherit())
            .output()?;

        if output.status.success() {
            return Ok(String::from_utf8(output.stdout).unwrap());
        } else {
            Err(Error::new(
                std::io::ErrorKind::Other,
                "Command execution failed",
            ))
        }
    }
}
