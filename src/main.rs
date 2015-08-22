extern crate hoedown;
extern crate handlebars;
extern crate rustc_serialize;

mod parsers;
mod generator;
mod utils;
mod processor;
use std::path::Path;
use handlebars::Handlebars;


fn main() {

    let layout_path = Path::new("default_layout.hbs");
    let layout_as_string: String = utils::read_file_to_string(layout_path).unwrap();
    let mut processor: Handlebars = Handlebars::new();

    processor.register_template_string(&utils::path_filename(layout_path).unwrap(), layout_as_string)
        .ok().unwrap();


    generator::generate("index.md", &mut processor, &utils::path_filename(layout_path).unwrap());
}
