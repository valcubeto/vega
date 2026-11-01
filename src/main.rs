#![allow(unused)] // Sometimes it gets annoying.

mod common;
mod text;
mod lex;
mod ir;
mod interpreter;

use common::*;

use lex::{ types::tokens::*, parsing::Lexer };
use ir::parsing::Parser;

fn main() -> anyhow::Result<()> {
    let input = Input {
        source: Box::from(Path::new("stdin")),
        data: Box::from(
            r#"
                // Comment
                println + a
                '\x7e'
                "Hello, world!\e"
            "#
        )
    };
    println!("(main) data = r#\"{}\"#", input.data);

    let (text, tokens) = Lexer::new(&input).lex()?;

    for token in tokens.iter() {
        println!("(main) token = {token:?}");
        if let TokenKind::String(sym) = token.kind {
            println!("             = String({:?})", text.lookup(sym).unwrap());
        }
    }

    println!();

    let statements = Parser::new(&input, tokens.as_ref()).make_ast()?;

    for stmt in statements.iter() {
        println!("(main) stmt = {stmt:#?}");
    }

    // let result = JIT::new(statements).eval_and_return_last()?;

    Ok(())
}
