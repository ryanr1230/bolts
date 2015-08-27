extern crate bolts;

include!("../config.rs");
fn main() {

	bolts::run(config::default_layout(), config::partial_path(), config::markdown_file()).unwrap();
}
