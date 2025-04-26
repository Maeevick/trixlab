use colored::*;

pub struct Recipe {
    name: String,
    ingredients: Vec<String>,
}

impl Recipe {
    pub fn new(name: &str, ingredients: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            ingredients: ingredients.iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn display(&self) {
        println!("Recipe: {}", self.name.magenta());
        println!("Ingredients:");
        for (i, ingredient) in self.ingredients.iter().enumerate() {
            println!("  {}. {}", i + 1, ingredient);
        }
    }
}
