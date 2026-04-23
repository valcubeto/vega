#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ProgramOptions {
    pub verbosity: Verbosity,
    pub colors: Coloring,
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
