extern crate hoedown;
extern crate handlebars;
extern crate rustc_serialize;
#[macro_use] extern crate maplit;

#[macro_use] mod common;
mod parsers;
mod generator;
use std::path::Path;
use handlebars::Handlebars;
use common::SiteGenResult;

pub fn run(layout_path_str: &'static str, partial_path_str: &'static str, markdown_file_str: &'static str) -> SiteGenResult<()> {
    let layout_path  = Path::new(layout_path_str);
    let mut processor: Handlebars = Handlebars::new();
    let partial_path = Path::new(partial_path_str);
    let partial_name: String = try!(common::register_template(partial_path, &mut processor));
    let parent_name: String = try!(common::register_template(layout_path, &mut processor));
    let markdown_file = Path::new(markdown_file_str);
    generator::generate(markdown_file, &mut processor, &partial_name, &parent_name);
    Ok(())
}
