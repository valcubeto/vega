pub struct TextContainer {
    data: Vec<u8>
}

impl TextContainer {
    pub fn with_capacity(capacity: usize) -> Self {
        TextContainer {
            data: Vec::with_capacity(capacity)
        }
    }

    pub fn push_str(&mut self, string: &str) -> &str {
        self.data.extend_from_slice(string.as_bytes());
        unsafe {
            str::from_utf8_unchecked(&self.data[self.data.len() - string.len()..])
        }
    }
}
