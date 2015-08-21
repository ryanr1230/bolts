extern crate hoedown;

mod parsers;
mod generator;

use generator::Generator;


fn main() {
    let generator: Generator = Generator::new("index.markdown");

    generator.generate();
}
