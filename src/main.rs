#![allow(unused)] // Sometimes it gets annoying.

mod common;
mod text;
mod lex;
mod ir;

use common::*;

use lex::{ types::tokens::*, parsing::Lexer };
use ir::parsing::Parser;

fn main() -> anyhow::Result<()> {
    let input = Input {
        source: Box::from(Path::new("stdin")),
        data: Box::from(
            r#"
                // Comment
                "Hello, world!"
            "#
        )
    };

    let (text, tokens) = Lexer::new(&input).lex()?;

    for token in tokens.iter() {
        println!("(main) token = {token:?}\n");
    }

    let statements = Parser::new(&input, tokens.as_ref()).make_ast()?;

    for stmt in statements.iter() {
        println!("(main) stmt = {stmt:?}\n");
    }

    // let result = JIT::new(statements).eval_and_return_last()?;

    Ok(())
}
