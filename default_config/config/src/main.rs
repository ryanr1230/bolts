extern crate bolts;
use bolts::Bolts;
include!("../config.rs");
fn main() {
	let bolts_runner = Bolts::new();
	bolts_runner.run(config::default_layout(), config::partial_paths(), config::markdown_files()).unwrap();
}
