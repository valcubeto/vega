use crate::flags::{ FromArgs, RawArgs };

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ProgramOptions {
    pub verbosity: Verbosity,
    pub colors: Coloring,
}

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

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Coloring {
    Disabled,
    Enabled
}
