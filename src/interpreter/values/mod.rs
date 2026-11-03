use crate::common::*;

pub mod modules;

#[derive(Debug)]
pub enum Value {
    Module(modules::Module),
    // Function(Function),
    // String(StringId)
}

impl Value {
    pub fn debug_depth_with(&self, text: &Interner, depth: usize) -> String {
        match self {
            Self::Module(x) => x.debug_depth_with(text, depth)
        }
    }
}
