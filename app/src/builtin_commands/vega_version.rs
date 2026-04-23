use prelude::terminal::debug;
use prelude::strings::{ NAME, VERSION };

pub fn print_version() {
    debug!("TODO: accept verbosity");
    println!("{NAME} v{VERSION}")
}
