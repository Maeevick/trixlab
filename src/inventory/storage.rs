use super::item::Item;
use colored::*;

pub struct Storage {
    name: String,
    items: Vec<Item>,
}

impl Storage {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn list_items(&self) {
        println!("Storage: {}", self.name.blue());
        if self.items.is_empty() {
            println!("  (empty)");
            return;
        }

        for (i, item) in self.items.iter().enumerate() {
            println!("  {}. {} (value: {})", i + 1, item.name(), item.value());
        }
    }
}
