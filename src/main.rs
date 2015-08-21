extern crate hoedown;

mod parser;

use parser::MarkdownParser;
use parser::Parser;


fn main() {
    let parser: MarkdownParser = Parser::new("index.markdown");

    parser.run();
}
