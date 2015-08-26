use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use parsers;
use parsers::ParserType;
use handlebars::Handlebars;
use processor::Layout;
use common::SiteGenResult;
use common::SiteGenError;

fn determine_type(path: &Path) -> SiteGenResult<(&'static str, ParserType)> {
    let extension_str = try_option_empty_error!(path.extension(), String::from("no parser available for no extension"));
    match extension_str.to_str().unwrap() {
        "markdown" | "md" => return Ok(("html", parsers::markdown_parse)),
        _ => return Err(SiteGenError::Other(String::from("no parser available for this extension"))),
    };
}

fn create_out_file<'a>(path: &Path, new_extension: &'a str) -> SiteGenResult<File> {
    let mut path_buf = path.to_path_buf();
    path_buf.set_extension(new_extension);
    let file = try!(File::create(&path_buf.as_path()));
    return Ok(file);
}

pub fn generate<'a>(path: &Path, processor: &mut Handlebars, layout_name: &'a str) {
    let parser = determine_type(&path).unwrap();
    let bytevec = parser.1(&path).unwrap();
    let mut out_file = create_out_file(path, parser.0).unwrap();
    let utf8_output = String::from_utf8(bytevec).unwrap();
    let data = Layout::new(utf8_output,"".to_string(), "".to_string());
    let to_write = processor.render(layout_name, &data);
    out_file.write(to_write.unwrap().as_ref()).unwrap();
}
