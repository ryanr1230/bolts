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

pub trait Parser {
    fn parse<'a>(&self, markdown_path: &Path) -> Result<Buffer>;

    fn new(path: &'static str) -> Self;

    #[allow(unused_must_use)]
    fn run<'a>(&self);

    fn create_out_file<'a>(&self, path: &Path, new_extension: &'a str) -> Result<File> {
        let mut path_buf = path.to_path_buf();
        path_buf.set_extension(new_extension);
        let file = try!(File::create(&path_buf.as_path()));
        return Ok(file);
    }

    fn path(&self) -> &'static str;
    fn new_extension(&self) -> &'static str;
}

pub struct MarkdownParser { markdown_path: &'static str, new_extension: &'static str}

impl MarkdownParser {}

impl Parser for MarkdownParser {
    fn new(path: &'static str) -> MarkdownParser {
        MarkdownParser { markdown_path: path, new_extension: "html" }
    }

    fn run<'a>(&self) {
        let markdown_path_str = self.path();
        let output_file_extension = self.new_extension();
        let markdown_path = Path::new(markdown_path_str);
        let markdown_buffer = self.parse(markdown_path).unwrap();
        let mut out_file = self.create_out_file(markdown_path, output_file_extension).unwrap();
        out_file.write(markdown_buffer.as_ref());
        ()
    }

    fn path(&self) -> &'static str {
        self.markdown_path
    }

    fn new_extension(&self) -> &'static str {
        self.new_extension
    }

    fn parse<'a>(&self, markdown_path: &Path) -> Result<Buffer> {
        let mut markdown_file = try!(File::open(&markdown_path));

        let mut input_markdown = String::new();
        try!(markdown_file.read_to_string(&mut input_markdown));
        let markdown_document = Markdown::new(&input_markdown);
        let mut html = Html::new(html::Flags::empty(), 0);

        return Ok(html.render(&markdown_document));
    }
}
