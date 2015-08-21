use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;
use hoedown::Markdown;
use hoedown::renderer::html::Html;
use hoedown::renderer::html;
use hoedown::renderer::Render;
use hoedown::Buffer;

pub type ParserType = fn(&Path) -> Result<Vec<u8>>;

pub fn markdown_parse<'p>(path:&Path) -> Result<Vec<u8>> {
    let mut markdown_file = try!(File::open(path));

    let mut input_markdown = String::new();
    try!(markdown_file.read_to_string(&mut input_markdown));
    let markdown_document = Markdown::new(&input_markdown);
    let mut html = Html::new(html::Flags::empty(), 0);
    let buffer: Buffer = html.render(&markdown_document);
    let byteslice: &[u8] = buffer.as_ref();
    let bytevec: Vec<u8> = Vec::from(byteslice);
    return Ok(bytevec);
}
