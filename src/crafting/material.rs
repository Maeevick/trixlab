#[derive(Debug, Clone)]
pub struct Material {
    name: String,
    rarity: Rarity,
    quantity: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Material {
    pub fn new(name: &str, rarity: Rarity, quantity: u32) -> Self {
        Self {
            name: name.to_string(),
            rarity,
            quantity,
        }
    }

    pub fn is_rare(&self) -> bool {
        self.rarity == Rarity::Rare
    }

    pub fn display_info(&self) -> String {
        format!(
            "{} ({}x) - {}",
            self.name,
            self.quantity,
            self.rarity_to_string()
        )
    }

    fn rarity_to_string(&self) -> &str {
        match self.rarity {
            Rarity::Common => "Common",
            Rarity::Uncommon => "Uncommon",
            Rarity::Rare => "Rare",
            Rarity::Epic => "Epic",
            Rarity::Legendary => "Legendary",
        }
    }
}
