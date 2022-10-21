use anyhow::Result;

pub struct App {
    pub filter: String,
    pub output: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            filter: "".to_string(),
            output: String::new(),
        }
    }
}

impl App {
    pub fn update_output(&mut self, output: std::process::Output) -> Result<()> {
        self.output = if output.status.success() {
            String::from_utf8(output.stdout)
        } else {
            String::from_utf8(output.stderr)
        }?;
        Ok(())
    }
}
