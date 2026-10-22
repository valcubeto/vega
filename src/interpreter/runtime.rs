use hashbrown::HashMap;
use slotmap::{ new_key_type, SlotMap };
use super::Value;

new_key_type! {
    struct ValueId;
}

#[allow(dead_code)]
pub struct Runtime<'rt> {
    pub env: HashMap<String, Value<'rt>>,
    pub stack: Vec<Value<'rt>>,
    pub heap: SlotMap<ValueId, Value<'rt>>,
}

impl<'rt> Runtime<'rt> {
    pub fn new() -> Self {
        Runtime {
            env: HashMap::new(),
            stack: Vec::new(),
            heap: SlotMap::with_key(),
        }
    }

    pub fn alloc(&mut self, value: Value<'rt>) -> ValueId {
        self.heap.insert(value)
    }
}

impl Default for Runtime<'rt> {
    fn default() -> Self {
        // FIXME:
        Self::new()
    }
}
