extern crate hoedown;
extern crate rumblebars;
extern crate rustc_serialize;

#[macro_use] mod common;
mod parsers;
mod generator;
use std::path::Path;
use common::SiteGenResult;
use rumblebars::Template;
use rumblebars::EvalContext;

pub fn run(layout_path_str: &'static str, partial_paths_str: Vec<&'static str>, markdown_files_str: Vec<&'static str>) -> SiteGenResult<()> {
    let mut context: EvalContext = EvalContext::new();

    let layout_path  = Path::new(layout_path_str);
    for partial_path_str in &partial_paths_str {
        let partial_path = Path::new(partial_path_str);
        let _ = try!(common::register_partial(partial_path, &mut context));
    }
    let layout_template: Template = try!(common::create_template(layout_path));
    for markdown_file_str in &markdown_files_str {
        let markdown_file = Path::new(markdown_file_str);
        try!(generator::generate(markdown_file, &layout_template, &mut context));
    }
    Ok(())
}
