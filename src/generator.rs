use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use parsers;
use parsers::ParserType;
use rumblebars;
use rumblebars::EvalContext;
use rumblebars::Template;
use common::SiteGenResult;
use common::SiteGenError;
use rustc_serialize;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::collections::HashMap;

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

pub fn generate<'a>(path: &Path,
                    layout_template: &Template,
                    context: &EvalContext) -> SiteGenResult<()>{
    let parser = determine_type(&path).unwrap();
    let bytevec = parser.1(&path).unwrap();
    let mut out_file = create_out_file(path, parser.0).unwrap();
    let utf8_output = String::from_utf8(bytevec).unwrap();

    let mut input_context_builder = HashMap::new();
    input_context_builder.insert("context", &utf8_output);
    let input_context = input_context_builder.clone();
    let input_context_string: String = try!(json::encode(&input_context));
    let input_context_json: Json = try!(Json::from_str(&input_context_string));
    try!(layout_template.eval(&input_context_json, &mut out_file, context));

    Ok(())
}
