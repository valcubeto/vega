use terminal::debug;
use strings::{ NAME, VERSION };

pub fn print_version() {
    debug!("TODO: accept verbosity");
    println!("{NAME} v{VERSION}")
}
