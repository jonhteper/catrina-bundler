use std::path::PathBuf;

use substring::Substring;

use crate::catrina::config::Config;

/// Js imports and exports struct
#[derive(Debug, Clone)]
pub struct Import {
    pub names: Vec<String>,
    pub path: String,
}

impl Import {
    pub fn new() -> Self {
        Import {
            names: vec![],
            path: "".to_string(),
        }
    }
}
