use args::flags::RawArgs;
use terminal::debug;

pub fn init_project(args: &RawArgs) {
    debug!("Initializing project with args {args:?}");
    todo!();
}

#[allow(dead_code)]
pub enum ProjectType {
    Binary,
    Library
}

#[allow(dead_code)]
pub struct ProjectOptions {
    kind: ProjectType,
    name: Option<String>
}
