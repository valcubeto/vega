mod keywords;
mod runtime;
mod error;
mod lexer;
mod parser;
mod interpreter;

use crate::{ runtime::Runtime, lexer::Lexer, parser::Parser, interpreter::{ Interpreter, Value } };
use std::{ path::Path, process::exit, io };
use crossterm::style::Stylize;

fn main() {
    let file = Path::new("<stdin>");
    // let data = r#"
    //     // Comment!
    //     // print("Hello, world!", '\n')
    //     // not not "hello" + "world" * (a - b) + x
    //     "hello, " + "world!"
    // "#;

    // let strings = StringRegistry::with_capacity(256);

    let mut rt = Runtime::new();

    loop {
        let line = ask(">> ".bold());
        let result = match eval(file, line.as_str(), &mut rt) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("{error}");
                continue;
            }
        };
        println!("{result}");
    }
}

fn eval<'a, 'rt>(file: &'a Path, data: &'a str, rt: &'rt mut Runtime<'rt>) -> error::Result<'a, Value<'rt>> {
    let tokens = Lexer::new(file, data, rt).lex()?;
    let ast = Parser::new(file, data, tokens.as_ref()).parse_global_scope()?;
    Interpreter::new(file, ast.as_ref(), rt).eval_scope()
}

fn ask(prompt: impl std::fmt::Display) -> String {
    use io::Write;
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    if let Ok(0) = io::stdin().read_line(&mut buf) {
        println!();
        exit(0);
    };
    buf
}
