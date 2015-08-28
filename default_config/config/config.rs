pub mod config {
    pub fn default_layout() -> &'static str {
        "default_layout.hbs"
    }
    pub fn markdown_files() -> Vec<&'static str> {
        vec!["index.md"]
    }
    pub fn partial_paths() -> Vec<&'static str> {
        vec!["header.hbs", "footer.hbs"]
    }
}
