use std::env::{ Args, args };
use std::fmt;

type Text = Box<str>;

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

fn main() {
    println!("-- Workers --");
    println!("# ArgParser: parse command line arguments");
    let args = ParsedArgs::from_args(args());
    println!("parsed_args = {:#?}", args);
    println!("# CommandLoader: load commands based on args");
}


pub trait FromArgs {
    fn from_args(args: Args) -> Self;
}

#[allow(dead_code)]
#[derive(fmt::Debug, Default)]
struct GlobalFlags {
    // quiet
    no_output: bool
}

// FlagParser:
// accept values by writing flag=value
// accept y,yes,true,n,no,false as bool values
trait ParseFlag {}

impl GlobalFlags {
    pub fn set(&mut self, raw_flag: &str) {
        // f=val
        // -flag=val
        println!("todo: flag {:?}", raw_flag);
    }
}

#[derive(fmt::Debug)]
struct CommandFlags;

#[allow(dead_code)]
#[derive(fmt::Debug)]
struct ParsedArgs {
    pub this: Text,
    pub command: Text,
    pub flags: GlobalFlags,
    pub command_flags: CommandFlags
}

impl FromArgs for ParsedArgs {
    fn from_args(mut args: Args) -> Self {
        let this = args.next()
            .expect("no args provided")
            .into_boxed_str();
        let mut flags = GlobalFlags::default();
        for arg in args.by_ref() {
            if arg.is_empty() || !arg.starts_with('-') {
                break;
            }
            flags.set(unsafe { arg.get_unchecked(1..) })
        }
        let command = args.next()
            .unwrap_or_default()
            .into_boxed_str();
        ParsedArgs {
            this,
            command,
            flags,
            command_flags: CommandFlags
        }
    }
}
