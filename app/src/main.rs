mod tests;
mod builtin_commands;

use terminal::debug;
use args::ParsedArgs;
#[cfg(debug_assertions)]
use strings::{ NAME, VERSION };

fn main() {
    debug!("Running {NAME} v{VERSION}.");

    debug!("Parsing command line arguments.");
    #[allow(unused)]
    let mut args = ParsedArgs::parse();
    debug!(args);

    match args.subcommand {
        Some(cmd) if is_help_command(&cmd) => {
            builtin_commands::help::print_help();
        }
        Some(cmd) if is_version_command(&cmd) => {
            builtin_commands::version::print_version();
        }
        Some(cmd) if cmd == "init" => {
            builtin_commands::init::init_project(&args.args);
        }
        Some(cmd) if cmd == "new" => {
            builtin_commands::new::create_project(&args.args);
        }
        Some(cmd) => {
            command_loader::run_external(&cmd, &args.args);
        }
        None => {
            builtin_commands::default::print_default();
        }
    }
    // let command = CommandLoader::load(args.subcommand);
    // command.run()
}

#[inline]
fn is_help_command(cmd: &str) -> bool {
    cmd == "help" || cmd == "--help" || cmd == "-h"
}

#[inline]
fn is_version_command(cmd: &str) -> bool {
    cmd == "version" || cmd == "--version" || cmd == "-V"
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
