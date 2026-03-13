mod prelude;
mod tests;

use args::Args;
use terminal::debug;

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    debug!("Running {NAME} v{VERSION}");

    debug!("ArgParser: parse command line arguments");
    #[allow(unused)]
    let mut args = Args::parse();
    debug!(args);

    debug!("CommandLoader: load subcommand based on args");
    // let command = CommandLoader::load(args.subcommand);
    // command.run()
}

/*
workers:
    Stage 0 - Setup:
        ArgParser
        CommandLoader {command_name}
        ConfigLoader {file}:
            create a config from a file 
        ConfigCreator:
            create the global config and
            command-specific config by
            parsing config files in the
            program's folder, in the current
            workspace, and from the parsed
            command line args
    Stage 1 ():
    Lexer {cfg, string}:
        converts a string into tokens
    Parser {cfg, tokens}:
        converts tokens into a syntax tree
    SemanticChecker {cfg, tree}:
        does type checking
    Optimizer {cfg, tree}
    IR_Generator {cfg, tree}:
        creates an IR (LLVM) from a syntax tree
    BinGenerator {cfg, IR}:
        produce a binary from the IR
    Runner {cfg, bin}:
        build and execute a binary
    ModuleResolver
commands:
    new {proj_name}
    init [template]
    lex {file}
    parse {file}
    compile {file}
    build
    run {file}
*/
