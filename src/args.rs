/// Arguments for CLI
pub struct CatrinaArgs<'a> {
    pub action: &'a str,
    pub filepath_1: &'a str,
    pub filepath_2: &'a str,
    pub filename: &'a str,
    pub skip: bool,
    pub yarn: bool,
    pub minify: bool,
}
