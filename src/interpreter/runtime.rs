use hashbrown::HashMap;
use slotmap::{ new_key_type, SlotMap };
use super::values::{ Value, modules::Module };
use crate::common::*;

new_key_type! {
    pub struct ValueId;
}

#[allow(dead_code)]
pub struct Runtime {
    pub env: HashMap<Symbol, Value>,
    pub stack: Vec<Value>,
    pub heap: SlotMap<ValueId, Value>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            env: hashmap! {},
            stack: vec![],
            heap: SlotMap::with_key(),
        }
    }

    pub fn alloc(&mut self, value: Value) -> ValueId {
        self.heap.insert(value)
    }

    pub fn default_with(text: &mut Interner) -> Self {
        // FIXME:
        let std_key = text.intern("std");
        let io_key = text.intern("io");
        Runtime {
            env: hashmap! {
                std_key => Value::Module(Module {
                    name: std_key,
                    members: hashmap! {
                        io_key => Value::Module(Module {
                            name: io_key,
                            members: hashmap! {
                                // "println" => Value::Function(Function {
                                //     name: "println",
                                //     args: Box::new([]),
                                //     body: Box::new([]),
                                // })
                            }
                        })
                    }
                })
            },
            stack: vec![],
            heap: SlotMap::with_key(),
        }
    }

    pub fn debug_with(&self, text: &Interner) -> String {
        self.debug_depth_with(text, 0)
    }

    fn debug_depth_with(&self, text: &Interner, depth: usize) -> String {
        let mut repr = String::from('{');
        let indent = &"    ".repeat(depth)[..];
        for (key, value) in self.env.iter() {
            repr.push('\n');
            repr.push_str(indent);
            repr.push_str("    ");
            // Sometimes I hate Rust
            repr.push_str(&(text.lookup(*key).underlined().to_string())[..]);
            repr.push_str(" = ");
            repr.push_str(&value.debug_depth_with(text, depth + 1)[..]);
        }
        repr.push('\n');
        repr.push_str(indent);
        repr.push('}');
        repr
    }
}
