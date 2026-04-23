use std::env;
use std::path::PathBuf;
use std::fs;
use std::process::Command;
use prelude::strings::NAME;
use prelude::terminal::*;
use prelude::types::*;

pub fn run_external(cmd_name: &str, args: SliceView<&str>) {
    // todo: workspace discovery, environment setup, etc
    let bin_name = format!("{NAME}-{cmd_name}");
    debug!("Searching command {bin_name:?}.");
    let bin_path = find_external(&bin_name);
    debug!(
        "The command does exist. Executing [{bin_path:?}, {args:?}]");
    let status = Result::unwrap_or_else(
        Command::new(&bin_path).args(args.as_ref()).status(),
        |error| fatal!("failed to execute {bin_name}. {error}.")
    );
    println!("{bin_name:?} exited with code {status}");
}

fn find_external(bin_name: &str) -> PathBuf {
    let path_var = Option::unwrap_or_else(
        env::var_os("PATH"),
        | | fatal!("failed to read the PATH variable.")
    );
    for dir in env::split_paths(&path_var) {
        let path = dir.join(bin_name);
        debug!("Trying {path:?}");
        if fs::metadata(&path).is_ok_and(|m| m.is_file()) {
            return path;
        }
    }
    fatal!("{bin_name:?} not found.");
}
