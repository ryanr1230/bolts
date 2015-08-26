extern crate hoedown;
extern crate handlebars;
extern crate rustc_serialize;

#[macro_use] mod common;
mod parsers;
mod generator;
mod processor;
use std::path::Path;
use handlebars::Handlebars;
use common::SiteGenResult;

pub fn run(layout_path_str: &'static str, markdown_file_str: &'static str) -> SiteGenResult<()> {
    let layout_path  = Path::new(layout_path_str);
    let mut processor: Handlebars = Handlebars::new();
    let layout_name: String = try!(common::register_template(layout_path, &mut processor));
    let markdown_file = Path::new(markdown_file_str);
    generator::generate(markdown_file, &mut processor, &layout_name);
    Ok(())
}
