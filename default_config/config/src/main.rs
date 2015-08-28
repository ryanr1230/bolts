extern crate bolts;

include!("../config.rs");
fn main() {

	bolts::run(config::default_layout(), config::partial_paths(), config::markdown_files()).unwrap();
}
