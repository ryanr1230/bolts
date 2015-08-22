use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::Path;
use parsers;
use parsers::ParserType;

fn determine_type(path: &Path) -> Result<(&'static str, ParserType)> {
    let extension_str = match path.extension() {
        Some(string) => string,
        None => return Err(Error::new(ErrorKind::Other, "no parser available for no extension")),
    };
    match extension_str.to_str().unwrap() {
        "markdown" => return Ok(("html", parsers::markdown_parse)),
        _ => return Err(Error::new(ErrorKind::Other, "no parser available for this extension")),
    };
}

fn create_out_file<'a>(path: &Path, new_extension: &'a str) -> Result<File> {
    let mut path_buf = path.to_path_buf();
    path_buf.set_extension(new_extension);
    let file = try!(File::create(&path_buf.as_path()));
    return Ok(file);
}

#[allow(unused_must_use)]
pub fn generate<'a>(path_str: &'static str) {
    let path = Path::new(path_str);
    let parser = determine_type(&path).unwrap();
    let bytevec = parser.1(&path).unwrap();
    let mut out_file = create_out_file(path, parser.0).unwrap();
    out_file.write(&bytevec[..]);
}
