use crate::error::Result;
use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdout, Command, Stdio},
};

pub struct NimbusServer {
    child: Child,
    reader: BufReader<ChildStdout>,
}
impl NimbusServer {
    pub fn new() -> Result<Self> {
        let mut child = Command::new("python")
            .arg("autophononimbus")
            .arg("server")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let reader = BufReader::new(child.stdout.take().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to capture stdout")
        })?);

        Ok(Self { child, reader })
    }

    pub fn syllabify(&mut self, word: &str) -> Option<String> {
        let stdin = self.child.stdin.as_mut()?;
        stdin.write_all(word.as_bytes()).ok()?;
        stdin.write_all(b"\n").ok()?;
        stdin.flush().ok()?;

        let mut output = String::new();
        self.reader.read_line(&mut output).ok()?;
        output = output.trim_end().to_string();

        Some(output)
    }

    pub fn stop(&mut self) -> Result<()> {
        self.child.kill()?;
        Ok(())
    }
}

impl Drop for NimbusServer {
    fn drop(&mut self) {
        self.stop().ok();
    }
}
