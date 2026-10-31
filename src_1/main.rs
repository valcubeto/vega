mod keywords;
pub mod vega;
mod lexer;
mod parser;
// mod interpreter;
mod strings;
mod common;

use common::*;

fn main() {
    let file: &Path = Path::new("<stdin>");
    // let data = r#"
    //     // Comment!
    //     // print("Hello, world!", '\n')
    //     // not not "hello" + "world" * (a - b) + x
    //     "hello, " + "world!"
    // "#;

    // let mut rt = Runtime::default();

    loop {
        let line = ask(">> ".bold());
        let result = match eval(file, line.as_str()) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("{error}");
                continue;
            }
        };
        // println!("{result}");
    }
}

fn eval<'a, 't>(file: &'a Path, data: &'a str) -> vega::Result<()> {
    let (text_data, tokens) = Lexer::new(file, data).lex()?;
    let ast = Parser::new(file, data, tokens.as_ref()).parse_global_scope()?;
    // Interpreter::new(file, ast.as_ref(), rt).eval_scope()
    Ok(())
}

fn ask(prompt: impl fmt::Display) -> String {
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
