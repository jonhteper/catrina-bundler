use std::path::PathBuf;

use substring::Substring;

use crate::catrina::config::Config;

/// Js imports and exports struct
#[derive(Debug)]
pub struct Import {
    pub names: Vec<String>,
    pub path: String,
}

impl Import {
    /// create a new Import from file with destructure imports
    /// # Important
    /// ```
    /// // imports.js
    /// // check no spaces in import list
    /// export {Alert,salert} from "./alerts/alert.js"
    /// ```
    pub fn new_from_line(line: String, config: &Config, canonicalize: bool) -> eyre::Result<Self> {
        let error_msj = "Error with export line un exports file";
        let s = line.split(" ").collect::<Vec<&str>>();
        let names = s.get(1).expect(&error_msj);
        let names_array = names
            .substring(1, names.len() - 1)
            .split(",")
            .collect::<Vec<&str>>();

        let mut names: Vec<String> = vec![];
        for n in names_array {
            names.push(n.trim().to_string());
        }

        let raw_path = s.get(3).expect(&error_msj);
        let raw_path = raw_path
            .replace("\"", "")
            .replace("./", "")
            .replace(";", "");
        let path_buf: PathBuf;

        if canonicalize {
            path_buf =
                PathBuf::from(format!("{}/{}", config.location_lib, raw_path)).canonicalize()?;
        } else {
            path_buf = PathBuf::from(raw_path);
        }

        let path = path_buf.to_str().expect(&error_msj);

        let import = Import {
            names,
            path: path.to_string(),
        };

        Ok(import)
    }
}
