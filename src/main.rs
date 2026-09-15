mod keywords;
mod strings;
mod reports;
mod tokens;
mod parser;

use crate::{
    strings::StringRegistry,
    tokens::Lexer,
    parser::Parser,
    reports::Reporter,
};
use std::{path::Path, process::exit};

fn main() {
    let file = Path::new("<testing>");
    let data = r#"
        // Comment!
        // print("Hello, world!", '\n')
        not not hello + world * (a - b + x
    "#;

    let strings = StringRegistry::with_capacity(data.len() / 4 * 3);
    let mut reporter = Reporter::new(file, data);

    let lexer = Lexer::new(data, &strings, &mut reporter);
    let tokens = match lexer.lex().filter(|_| !reporter.errored()) {
        None => {
            eprintln!();
            eprintln!("Stopped lexing due to previous errors");
            exit(1);
        }
        Some(tokens) => {
            reporter.print_all();
            tokens
        }
    };

    #[cfg(debug_assertions)]
    println!("(main) tokens = {:#?}", tokens);

    let parser = Parser::new(tokens.as_ref(), /* &strings, */ &mut reporter);
    let ast = match parser.parse_mod().filter(|_| !reporter.errored()) {
        None => {
            eprintln!();
            eprintln!("Stopped parsing due to previous errors");
            exit(1);
        }
        Some(tokens) => {
            reporter.print_all();
            tokens
        }
    };

    handle_reports(reporter.view());
    // parser.finish();

    #[cfg(debug_assertions)]
    println!("(main) ast = {:#?}", ast);
}

fn handle_reports(reports: &[reports::Report]) {
    for report in reports.iter() {
        eprintln!("{}", report);
    }
}
