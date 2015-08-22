use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::Path;
use parsers;
use parsers::ParserType;
use handlebars::Handlebars;
use processor::Layout;

fn determine_type(path: &Path) -> Result<(&'static str, ParserType)> {
    let extension_str = match path.extension() {
        Some(string) => string,
        //TODO: new Error type
        None => return Err(Error::new(ErrorKind::Other, "no parser available for no extension")),
    };
    match extension_str.to_str().unwrap() {
        "markdown" => return Ok(("html", parsers::markdown_parse)),
        //TODO: new Error type
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
pub fn generate<'a>(path_str: &'static str, processor: &mut Handlebars, layout_name: &'a str) {
    let path = Path::new(path_str);
    let parser = determine_type(&path).unwrap();
    let bytevec = parser.1(&path).unwrap();
    let mut out_file = create_out_file(path, parser.0).unwrap();
    let utf8_output = String::from_utf8(bytevec).unwrap();
    let data = Layout::new(utf8_output,"footer".to_string());
    let to_write = processor.render(layout_name, &data);
    out_file.write(to_write.unwrap().as_ref());
}
