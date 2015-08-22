extern crate hoedown;

mod parsers;
mod generator;



fn main() {
    generator::generate("index.markdown");
}
