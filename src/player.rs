/// Represents the player in the game
#[derive(Debug, Clone)]
pub struct Player {
    /// The current room where the player is located
    pub location: String,
    /// Items the player has collected
    pub inventory: Vec<String>,
}

impl Player {
    /// Creates a new player at the specified starting location
    pub fn new(starting_location: &str) -> Self {
        Player {
            location: starting_location.to_string(),
            inventory: Vec::new(),
        }
    }

    /// Add an item to the player's inventory
    pub fn take_item(&mut self, item: &str) {
        self.inventory.push(item.to_string());
    }

    /// Check if player has the specified item
    pub fn has_item(&self, item: &str) -> bool {
        self.inventory.iter().any(|i| i.to_lowercase() == item.to_lowercase())
    }

    /// Display the player's inventory
    pub fn display_inventory(&self) -> String {
        if self.inventory.is_empty() {
            "Your inventory is empty.".to_string()
        } else {
            let mut inventory_list = String::from("You are carrying:\n");
            for item in &self.inventory {
                inventory_list.push_str(&format!("- {}\n", item));
            }
            inventory_list
        }
    }
}
