#[derive(Debug, Clone)]
pub struct Item {
    name: String,
    value: u32,
}

impl Item {
    pub fn new(name: &str, value: u32) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
