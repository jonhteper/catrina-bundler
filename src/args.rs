/// Arguments for CLI
pub struct CatrinaArgs<'a> {
    pub action: &'a str,
    pub param: &'a str,
    pub skip: bool,
    pub yarn: bool,
}
