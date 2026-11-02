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
                a + b * c and d
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

    let statements = match Parser::new(&input, tokens.as_ref()).make_ast() {
        Ok(statements) => statements,
        Err((pos, err)) => {
            for cause in err.chain() {
                eprintln!("{cause:#?}");
            }
            let Position(start, end) = pos;
            let (li, co) = line_and_column(&input.data, start as usize);
            eprintln!("    at {}:{}:{}", input.source.display(), li, co);
            eprintln!();
            eprintln!("{}", &input.data[start as usize..end as usize]);
            eprintln!("{}", "^".repeat(end.saturating_sub(start) as usize).red());
            exit(1);
        }
    };

    for stmt in statements.iter() {
        println!("(main) stmt = {:}", stmt.kind.debug_with(&text));
    }

    // let result = Interpreter::new(statements).eval_and_return_last()?;

    Ok(())
}

pub fn line_and_column(contents: &str, start: usize) -> (u16, u16) {
    let (mut line, mut column) = (1, 1);
    for (i, ch) in contents.chars().enumerate() {
        if i > start {
            break;
        }
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}
