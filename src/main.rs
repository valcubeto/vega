mod keywords;
mod strings;
mod tokens;
mod parser;

use crate::{
    strings::StringRegistry,
    tokens::Lexer,
    parser::Parser,
};
use std::path::Path;

fn main() {
    let file = Path::new("<testing>");
    let data = r#"
        // Comment!
        println("Hello, world!")
    "#;

    let strings = StringRegistry::with_capacity(data.len() / 4 * 3);
    let mut lexer = Lexer::new(data, &strings);
    #[allow(dead_code)]
    let tokens = lexer.lex().unwrap();
    #[cfg(debug_assertions)]
    println!("{:#?}", tokens);

    let mut parser = Parser::new(file, &strings, tokens.as_slice());
    let ast = parser.parse().unwrap();
}
