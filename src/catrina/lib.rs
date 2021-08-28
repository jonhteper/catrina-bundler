use crate::catrina::utils::{bin_dir, getwd};
use crate::catrina::VERSION_APP;
use eyre::Result;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct StdLib {
    npm: bool,
}

impl StdLib {
    pub fn new(default: bool) -> Self {
        StdLib { npm: default }
    }

    ///    install catrina package from npm
    pub fn install(&self) -> Result<()> {
        if self.npm {
            StdLib::install_by_npm()
                .expect("Error using npm. Please make sure you have the program installed")
        } else {
            StdLib::install_by_yarn()
                .expect("Error using yarn. Please make sure you have the program installed")
        }

        Ok(())
    }

    fn install_by_npm() -> Result<()> {
        /* exec
         * npm init -y
         * npm install --save catrina
         */
        let _output = Command::new("npm").args(&["init", "-y"]).output()?;
        let _output = Command::new("npm")
            .args(&["install", "--save", "catrina"])
            .output()?;

        Ok(())
    }

    fn install_by_yarn() -> Result<()> {
        /* exec
         * yarn init -y
         * yarn add catrina
         */
        let _output = Command::new("yarn").args(&["init", "-y"]).output()?;
        let _output = Command::new("yarn").args(&["add", "catrina"]).output()?;

        Ok(())
    }
}
