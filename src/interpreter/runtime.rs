use hashbrown::HashMap;
use bumpalo::Bump;
use super::Value;

#[allow(dead_code)]
pub struct Runtime<'s, 'h> {
    pub env: HashMap<&'h str, Value<'s, 'h>>,
    pub stack: Vec<Value<'s, 'h>>,
    pub heap: Bump,
}

impl<'s, 'h> Runtime<'s, 'h> {
    pub fn new() -> Self {
        Runtime {
            env: HashMap::new(),
            heap: Bump::new(),
            stack: Vec::new()
        }
    }
}
