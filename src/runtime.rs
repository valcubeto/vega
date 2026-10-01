use hashbrown::HashMap;
use bumpalo::Bump;
use super::Value;

#[allow(dead_code)]
pub struct Runtime<'rt> {
    pub env: HashMap<&'rt str, Value<'rt>>,
    pub stack: Vec<Value<'rt>>,
    pub heap: Bump,
}

impl<'rt> Runtime<'rt> {
    pub fn new() -> Self {
        Runtime {
            env: HashMap::new(),
            heap: Bump::new(),
            stack: Vec::new()
        }
    }

    pub fn heap_str(&self, string: &str) -> &str {
        self.heap.alloc_str(string)
    }
}
