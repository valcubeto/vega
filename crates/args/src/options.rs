use crate::flags::{ FromArgs, RawArgs };

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ProgramOptions {
    pub verbosity: Verbosity,
    pub colors: Coloring,
}

// TODO: Vec::drain_filter to remove flags
// TODO: use the chrono
impl FromArgs for ProgramOptions {
    fn from_args(_args: &mut RawArgs) -> Self {
        ProgramOptions {
            verbosity: Verbosity::default(),
            colors: Coloring::Disabled
        }
    }
}

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Default)]
pub enum Verbosity {
    Quiet,
    #[default]
    Normal,
    Verbose
}

// I'm gonna be honest with you.
// I don't like using bool inside structs.
#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Coloring {
    Disabled,
    Enabled
}
