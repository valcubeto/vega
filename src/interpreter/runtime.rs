use hashbrown::HashMap;
use slotmap::{ new_key_type, SlotMap };
use super::values::Value;
use crate::common::*;

new_key_type! {
    pub struct ValueId;
}

#[allow(dead_code)]
pub struct Runtime {
    pub env: HashMap<String, Value>,
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
}

impl Default for Runtime {
    fn default() -> Self {
        // FIXME:
        Runtime {
            env: hashmap! {
                // "std" => Value::Module(Module {
                //     name: "std",
                //     members: hashmap! {
                //         "io" => Value::Module(Module {
                //             name: "io",
                //             members: hashmap! {
                //                 "println" => Value::Function(Function {
                //                     name: "println",
                //                     args: Box::new([]),
                //                     body: Box::new([]),
                //                 })
                //             }
                //         })
                //     }
                // })
            },
            stack: vec![],
            heap: SlotMap::with_key(),
        }
    }
}
