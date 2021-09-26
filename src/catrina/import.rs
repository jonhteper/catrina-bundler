use std::path::PathBuf;

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

    pub fn path_buf(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }
}
