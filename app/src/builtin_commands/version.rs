use terminal::debug;
use strings::{ NAME, VERSION };

pub fn print_version() {
    debug!("todo: use the -v flag");
    println!("{NAME} v{VERSION}")
}
