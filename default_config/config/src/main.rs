extern crate bolts;

use std::path::Path;
include!("../config.rs");
fn main() {

	let default_layout = Path::new(config::default_layout());
	bolts::run(&default_layout, config::markdown_file());
}
