mod keywords;
mod strings;
mod error;
mod lexer;
mod parser;
mod interpreter;

use crate::{ strings::StringRegistry, lexer::Lexer, parser::Parser, interpreter::{ Interpreter, Runtime, Value } };
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

    // let strings = StringRegistry::with_capacity(data.len() / 4 * 3);
    let strings = StringRegistry::with_capacity(256);

    // let tokens = match Lexer::new(file, data, &strings).lex() {
    //     Ok(tokens) => tokens,
    //     Err(error) => {
    //         eprintln!("{error}");
    //         exit(1);
    //     }
    // };

    // #[cfg(debug_assertions)]
    // {
    //     println!("(main) tokens = {:#?}", tokens);
    //     println!("(main) {} {} {}", "Correctly parsed".green(), tokens.len().to_string().green(), "tokens".green());
    // }

    // #[cfg(debug_assertions)]
    // println!("(main) ast = {:#?}", ast);

    let mut rt = Runtime::new();
    // let interpreter = Interpreter::new(file, rt, ast.as_ref());
    // let _result = match interpreter.eval_scope() {
    //     Ok(result) => result,
    //     Err(error) => {
    //         eprintln!("{error}");
    //         exit(1);
    //     }
    // };

    loop {
        let line = ask(">> ".bold());
        let result = match eval(file, line.as_str(), &strings, &mut rt) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("{error}");
                continue;
            }
        };
        println!("{result}")
    }
}

fn eval<'a, 's, 'h>(file: &'a Path, data: &'a str, strings: &'s StringRegistry, rt: &'h mut Runtime<'s, 'h>) -> error::Result<'a, Value<'s, 'h>> {
    let tokens = Lexer::new(file, data, strings).lex()?;
    let ast = Parser::new(file, data, tokens.as_ref()).parse_global_scope()?;
    Interpreter::new(file, rt, ast.as_ref()).eval_scope()
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
