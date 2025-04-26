use colored::*;
use trixlab::crafting::material::{Material, Rarity};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filter_rarity = if args.len() > 1 {
        match args[1].to_lowercase().as_str() {
            "common" => Some(Rarity::Common),
            "uncommon" => Some(Rarity::Uncommon),
            "rare" => Some(Rarity::Rare),
            "epic" => Some(Rarity::Epic),
            "legendary" => Some(Rarity::Legendary),
            _ => None,
        }
    } else {
        None
    };

    println!("{}", "=== MATERIAL VIEWER ===".blue().bold());

    if let Some(ref rarity) = filter_rarity {
        println!("Filtering by rarity: {:?}", rarity);
    }

    let materials = vec![
        Material::new("Copper", Rarity::Common, 42),
        Material::new("Lunar Steel", Rarity::Uncommon, 13),
        Material::new("Arcanum Crystal", Rarity::Rare, 5),
        Material::new("Star Essence", Rarity::Epic, 2),
        Material::new("Meteorite Metal", Rarity::Legendary, 1),
    ];

    let filtered_materials = if let Some(ref rarity) = filter_rarity {
        materials
            .into_iter()
            .filter(|m| m.get_rarity() == *rarity)
            .collect()
    } else {
        materials
    };

    if filtered_materials.is_empty() {
        println!("No materials found with this rarity.");
    } else {
        for (i, material) in filtered_materials.iter().enumerate() {
            println!("  {}. {}", i + 1, material.display_info());
        }
    }
}
