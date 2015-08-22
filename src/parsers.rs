use std::io::Result;
use std::path::Path;
use hoedown::Markdown;
use hoedown::renderer::html::Html;
use hoedown::renderer::html;
use hoedown::renderer::Render;
use hoedown::Buffer;
use utils;
pub type ParserType = fn(&Path) -> Result<Vec<u8>>;

pub fn markdown_parse<'p>(path:&Path) -> Result<Vec<u8>> {
    let input_markdown: String = try!(utils::read_file_to_string(path));
    let markdown_document = Markdown::new(&input_markdown);
    let mut html = Html::new(html::Flags::empty(), 0);
    let buffer: Buffer = html.render(&markdown_document);
    let byteslice: &[u8] = buffer.as_ref();
    let bytevec: Vec<u8> = Vec::from(byteslice);
    return Ok(bytevec);
}
