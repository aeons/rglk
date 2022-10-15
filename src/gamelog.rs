pub struct GameLog {
    pub entries: Vec<String>,
}

impl GameLog {
    pub fn append(&mut self, msg: impl ToString) {
        self.entries.push(msg.to_string())
    }
}

impl Default for GameLog {
    fn default() -> Self {
        Self {
            entries: vec!["Welcome to rglk".into()],
        }
    }
}
