extern crate hoedown;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;
use std::io;
use hoedown::Markdown;
use hoedown::renderer::html::Html;
use hoedown::renderer::html;
use hoedown::renderer::Render;
use hoedown::Buffer;
#[allow(unused_must_use)]
fn main() {
    let markdown_path : &'static str = "index.markdown";
    let markdown_buffer = match parse_markdown(markdown_path) {
        Err(why) => panic!("Couldn't read from markdown file {}: {}", markdown_path,
                           Error::description(&why)),
        Ok(markdown_parsed) => markdown_parsed,
    };
    let mut out_file = match create_out_file(markdown_path) {
        Err(why) => panic!("Couldn't create output file: {}", Error::description(&why)),
        Ok(file) => file,
    };
    out_file.write(markdown_buffer.as_ref());
}

fn create_out_file<'a>(markdown_path_str: &'a str) -> Result<File> { 
    let output_filename = try!(convert_filename(markdown_path_str));
    let file = try!(File::create(output_filename));
    return Ok(file);
}

fn convert_filename<'a>(markdown_filename: &'a str) -> Result<String> {
    let string_filename = String::from(markdown_filename);
    let index = match string_filename.rfind(".markdown") {
        Some(i) => i,
        //TODO: make new error type, don't use io::Error
        None => return Err(io::Error::new(io::ErrorKind::Other, ".markdown not found in filepath name")),
    };
    let actual_name = &string_filename[..index];
    return Ok(format!("{}{}",actual_name,".html"));
}


fn parse_markdown<'a>(markdown_path_str: &'a str) -> Result<Buffer> {
    let markdown_path = Path::new(markdown_path_str);

    let mut markdown_file = try!(File::open(&markdown_path));
   
    let mut input_markdown = String::new(); 
    try!(markdown_file.read_to_string(&mut input_markdown));
    let markdown_document = Markdown::new(&input_markdown);
    let mut html = Html::new(html::Flags::empty(), 0);

    return Ok(html.render(&markdown_document));
}
