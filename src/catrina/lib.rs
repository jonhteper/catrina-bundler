use self::fs_extra::dir;
use crate::catrina::utils::bin_dir;
use crate::catrina::VERSION_APP;
use std::fs;
use std::path::PathBuf;

extern crate fs_extra;

pub struct StdLib {
    pub version: String,
    current_path: PathBuf,
}

impl StdLib {
    pub fn new(version: &str, path: PathBuf) -> StdLib {
        StdLib {
            version: version.to_string(),
            current_path: path,
        }
    }

    pub fn get(&self) -> Result<(), fs_extra::error::Error> {
        let mut path_from = bin_dir();
        path_from.push("lib");
        path_from.push(&self.version);

        let mut from = Vec::new();
        from.push(path_from.to_str().unwrap());

        fs_extra::copy_items(
            &from,
            &self.current_path.to_str().unwrap(),
            &dir::CopyOptions::new(),
        )?;

        &self.rename_after_copy()?;

        Ok(())
    }

    fn rename_after_copy(&self) -> std::io::Result<()> {
        let mut from_path = PathBuf::from(&self.current_path);
        from_path.push(VERSION_APP);
        let mut to_path = PathBuf::from(&self.current_path);
        to_path.push("lib");

        fs::rename(from_path, to_path)?;
        Ok(())
    }
}
