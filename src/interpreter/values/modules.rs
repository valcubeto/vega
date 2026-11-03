use crate::common::*;
use super::Value;

#[derive(Debug)]
pub struct Module {
    pub name: Symbol,
    pub members: HashMap<Symbol, Value>
}

impl Module {
    pub fn debug_depth_with(&self, text: &Interner, depth: usize) -> String {
        let mut repr = format!("{} {} {{", "mod".magenta(), text.lookup(self.name));
        let indent = &"    ".repeat(depth)[..];
        for (key, value) in self.members.iter() {
            repr.push_str(&format!("\n{}    {} = ", indent, text.lookup(*key).underlined())[..]);
            repr.push_str(&value.debug_depth_with(text, depth + 1)[..]);
        }
        repr.push('\n');
        repr.push_str(indent);
        repr.push('}');
        repr
    }
}
