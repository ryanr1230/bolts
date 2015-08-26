extern crate bolts;

include!("../config.rs");
fn main() {

	bolts::run(config::default_layout(), config::markdown_file()).unwrap();
}
