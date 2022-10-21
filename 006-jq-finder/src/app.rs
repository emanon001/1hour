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
