extern crate bolts;

use std::path::Path;
include!("../config.rs");
fn main() {

	bolts::run(config::default_layout(), config::markdown_file());
}
