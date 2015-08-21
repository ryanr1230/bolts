extern crate hoedown;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;
use hoedown::Markdown;
use hoedown::renderer::html::Html;
use hoedown::renderer::html;
use hoedown::renderer::Render;
use hoedown::Buffer;

#[allow(unused_must_use)]
fn main() {
    let markdown_path_str: &'static str = "index.markdown";
    let output_file_extension: &'static str = "html";
    let markdown_path = Path::new(markdown_path_str);
    let markdown_buffer = parse_markdown(markdown_path).unwrap(); 
    let mut out_file = create_out_file(markdown_path, output_file_extension).unwrap();
    out_file.write(markdown_buffer.as_ref());
}

fn create_out_file<'a>(path: &Path, new_extension: &'a str) -> Result<File> {
      let mut path_buf = path.to_path_buf();
      path_buf.set_extension(new_extension);
      let file = try!(File::create(&path_buf.as_path()));
      return Ok(file);
}

fn parse_markdown<'a>(markdown_path: &Path) -> Result<Buffer> {
    let mut markdown_file = try!(File::open(&markdown_path));
   
    let mut input_markdown = String::new(); 
    try!(markdown_file.read_to_string(&mut input_markdown));
    let markdown_document = Markdown::new(&input_markdown);
    let mut html = Html::new(html::Flags::empty(), 0);

    return Ok(html.render(&markdown_document));
}
