use crate::Act;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Catch {
    #[serde(default)]
    pub err: Option<String>,
    #[serde(default)]
    pub then: Vec<Act>,
}

impl Catch {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_err(mut self, err: &str) -> Self {
        self.err = Some(err.to_string());
        self
    }

    pub fn with_then(mut self, build: fn(Vec<Act>) -> Vec<Act>) -> Self {
        let stmts = Vec::new();
        self.then = build(stmts);

        self
    }
}
