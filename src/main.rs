extern crate hoedown;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io;
use hoedown::Markdown;
use hoedown::renderer::html::Html;
use hoedown::renderer::html;
use hoedown::renderer::Render;
use hoedown::Buffer;

fn main() {
    let markdown_path : &'static str = "index.markdown";
    match parse_markdown(markdown_path) {
        Err(why) => panic!("Couldn't read from markdown file {}: {}", markdown_path,
                           Error::description(&why)),
        Ok(markdown_parsed) => print!("{}", markdown_parsed),
    }
}


fn parse_markdown<'a>(markdown_path_str: &'a str) -> io::Result<String> {
    let markdown_path = Path::new(markdown_path_str);

    let mut markdown_file = try!(File::open(&markdown_path));
   
    let mut input_markdown = String::new(); 
    try!(markdown_file.read_to_string(&mut input_markdown));
    let markdown_document = Markdown::new(&input_markdown);
    let mut html = Html::new(html::Flags::empty(), 0);

    let rm = html.render(&markdown_document);
    let rm_str = rm.to_str();
    let rendered_markdown = match rm_str {
      Err(why)  => return Err(io::Error::new(io::ErrorKind::Other, Error::description(&why))),
      Ok(str) => String::from(str),
    };
    return Ok(rendered_markdown);
}
