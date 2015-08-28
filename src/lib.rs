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

pub fn run(layout_path_str: &'static str, partial_path_str: &'static str, markdown_file_str: &'static str) -> SiteGenResult<()> {
    let mut context: EvalContext = EvalContext::new();

    let layout_path  = Path::new(layout_path_str);
    let partial_path = Path::new(partial_path_str);
    let markdown_file = Path::new(markdown_file_str);
    let layout_template: Template = try!(common::create_template(layout_path));
    let _ = try!(common::register_partial(partial_path, &mut context));
    try!(generator::generate(markdown_file, &layout_template, &mut context));
    Ok(())
}
