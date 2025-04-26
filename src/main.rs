mod crafting;
mod inventory;
mod utils;

use chrono::Local;
use colored::*;
use crafting::recipe::Recipe;
use inventory::item::Item;
use inventory::storage::Storage;
use utils::random::generate_unique_id;

fn main() {
    println!("{}", "=== TRIXLAB WORKSHOP ===".green().bold());
    println!("Startup: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!("Session ID: {}", generate_unique_id());

    let mut storage = Storage::new("Main Storage");
    let gem = Item::new("Arcane Gem", 5);
    let metal = Item::new("Enchanted Metal", 10);

    storage.add_item(gem);
    storage.add_item(metal);

    println!("\n{}", "Inventory:".yellow());
    storage.list_items();

    let recipe = Recipe::new("Protection Amulet", vec!["Arcane Gem", "Enchanted Metal"]);
    println!("\n{}", "Available Recipe:".yellow());
    recipe.display();

    println!("\n{}", "=========================".green().bold());
}
