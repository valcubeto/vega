use std::env;
use std::path::PathBuf;
use std::process::Command;
use strings::NAME;
use terminal::debug;

pub fn run_external(cmd_name: &str, args: &[String]) {
    // todo: workspace discovery, environment setup, etc
    let bin_name = format!("{NAME}-{cmd_name}");
    debug!("Searching command {bin_name:?}.");
    let bin_path = find_external(&bin_name);
    debug!("The command does exist. Executing <{bin_path:?} {}>...", args.join(" "));
    let _command = Command::new(&bin_path)
        .args(args)
        .status()
        .expect("failed to execute (TODO: proper error)");
}

fn find_external(bin_name: &str) -> PathBuf {
    let path_var = env::var_os("PATH")
        .expect("failed to read PATH (TODO: proper error)");
    for dir in env::split_paths(&path_var) {
        let path = dir.join(bin_name);
        debug!("Trying {path:?}");
        if std::fs::metadata(&path).is_ok()/* _and(|m| m.is_file()) */ {
            return path;
        }
    }
    todo!("command not found");
}
