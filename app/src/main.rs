mod tests;
mod builtin_commands;

use prelude::types::*;
use prelude::terminal::*;
use args::ParsedArgs;
#[cfg(debug_assertions)]
use prelude::strings::{ NAME_CAPITALIZED, VERSION };

fn main() {
    debug!("Running {NAME_CAPITALIZED} v{VERSION}.");

    let args = ParsedArgs::parse();
    debug!(args);

    match args.subcommand {
        Some(cmd) if is_help_command(&cmd) => {
            builtin_commands::vega_help::print_help();
        }
        Some(cmd) if is_version_command(&cmd) => {
            builtin_commands::vega_version::print_version();
        }
        Some(cmd) if cmd.as_ref() == "init" => {
            builtin_commands::vega_init::init_project(args.rest.as_ref());
        }
        Some(cmd) if cmd.as_ref() == "new" => {
            builtin_commands::vega_new::create_project(args.rest.as_ref());
        }
        Some(cmd) => {
            let args = normalize_str_slice(args.rest.as_ref());
            command_loader::run_external(&cmd, &args);
        }
        None => {
            builtin_commands::default::print_default();
        }
    }
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
 * Workers:
 *     Stage 0 - Setup:
 *         ArgParser
 *         ConfigLoader {file}:
 *             Create a config from a file 
 *         ConfigCreator:
 *             Create the global config and
 *             command-specific config by
 *             parsing config files in the
 *             program's folder, in the current
 *             workspace, and from the parsed
 *             command line args.
 *             This includes loading the 
 *         CommandLoader {command_name}
 *     Stage 1:
 *     Lexer {cfg, string}:
 *         Convert a string into tokens
 *     Parser {cfg, tokens}:
 *         Convert tokens into a syntax tree
 *     ModuleResolver
 *     SemanticChecker {cfg, tree}:
 *         Do type checking
 *     Optimizer {cfg, tree}
 *     IR_Generator {cfg, tree}:
 *         Create an IR (LLVM?) from a syntax tree
 *     BinGenerator {cfg, IR}:
 *         Produce a binary from the IR
 *     Runner {cfg, bin}:
 *         Build and execute a binary
 * Commands:
 *     new {proj_name}
 *     init [template]
 *     lex {file}
 *     parse {file}
 *     compile {file}
 *     build
 *     run {file}
 */
