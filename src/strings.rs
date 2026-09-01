pub struct StringRegistry {
    data: bumpalo::Bump
}

impl StringRegistry {
    pub fn with_capacity(cap: usize) -> Self {
        StringRegistry { data: bumpalo::Bump::with_capacity(cap) }
    }
    pub fn push_str(&self, string: &str) -> &str {
        self.data.alloc_str(string)
    }
}
