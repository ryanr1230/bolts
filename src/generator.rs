use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::Path;
use parsers;
use parsers::ParserType;

pub struct Generator { path: &'static str }

impl Generator {
    pub fn new(path: &'static str) -> Generator {
        Generator { path: path }
    }

    fn determine_type(&self, path: &Path) -> Result<(&'static str, ParserType)> {
        let extension_str = match path.extension() {
            Some(string) => string,
            None => return Err(Error::new(ErrorKind::Other, "no parser available for no extension")),
        };
        match extension_str.to_str().unwrap() {
            "markdown" => return Ok(("html", parsers::markdown_parse)),
            _ => return Err(Error::new(ErrorKind::Other, "no parser available for this extension")),
        };
    }

    fn create_out_file<'a>(&self, path: &Path, new_extension: &'a str) -> Result<File> {
        let mut path_buf = path.to_path_buf();
        path_buf.set_extension(new_extension);
        let file = try!(File::create(&path_buf.as_path()));
        return Ok(file);
    }

    #[allow(unused_must_use)]
    pub fn generate<'a>(&self) {
        let path_str = self.path();
        let path = Path::new(path_str);
        let parser = self.determine_type(&path).unwrap();
        let bytevec = parser.1(&path).unwrap();
        let mut out_file = self.create_out_file(path, parser.0).unwrap();
        out_file.write(&bytevec[..]);
    }

    fn path(&self) -> &'static str {
        self.path
    }
}
