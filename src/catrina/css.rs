use eyre::Result;
use fs_extra::dir;
use crate::catrina::config::Config;
use crate::catrina::utils::file_to_string;
use std::path::PathBuf;
use std::fs::File;

pub struct Parser {
    config: Config,
    fonts_relation: Vec<RelationCSSFont>
}

impl Parser {

    pub fn new(config: Config) -> Result<Self> {
        let fonts_relation = Parser::read_fonts_relation(&config)?;
        Ok(Parser{
            config,
            fonts_relation
        })
    }

    fn read_fonts_relation(config: &Config) -> Result<Vec<RelationCSSFont>> {
        let mut file_location = PathBuf::from(&config.location_lib);
        file_location.push("css-fonts-relation.json");
        let mut fonts_relation: Vec<RelationCSSFont> = vec![];

        let file = File::open(&file_location)?;

        let data = file_to_string(file)?;

        fonts_relation = serde_json::from_str(&data)?;

        Ok(fonts_relation)

    }
}


pub struct RelationCSSFont {
    pub name: String,
    pub path: String
}

impl RelationCSSFont {
    pub fn get_font(&self, config: &Config) -> Result<()> {
        let options = dir::CopyOptions::new();
        let mut from_paths = Vec::new();
        from_paths.push(&self.path);

        fs_extra::copy_items(&from_paths, &config.deploy_path, &options)?;

        Ok(())
    }
}