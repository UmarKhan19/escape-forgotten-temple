use std::collections::HashMap;

/// Represents the possible directions a player can move
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Converts a string to a Direction enum value
    pub fn from_string(s: &str) -> Option<Direction> {
        match s.to_lowercase().as_str() {
            "north" => Some(Direction::North),
            "east" => Some(Direction::East),
            "south" => Some(Direction::South),
            "west" => Some(Direction::West),
            _ => None,
        }
    }

    /// Converts a Direction enum value to a string
    pub fn to_string(&self) -> &str {
        match self {
            Direction::North => "north",
            Direction::East => "east",
            Direction::South => "south",
            Direction::West => "west",
        }
    }
}

/// Represents a room in the game
#[derive(Debug)]
pub struct Room {
    /// Unique name/identifier for the room
    pub name: String,
    /// Description of the room shown to the player
    pub description: String,
    /// Available exits from the room
    pub exits: HashMap<Direction, String>,
    /// Items that can be found in the room
    pub items: Vec<String>,
    /// Flag indicating if this room is the winning exit
    pub is_exit: bool,
    /// Item required to win if this is an exit room
    pub required_item: Option<String>,
}

impl Room {
    /// Creates a new room with the given name and description
    pub fn new(name: &str, description: &str, is_exit: bool, required_item: Option<String>) -> Self {
        Room {
            name: name.to_string(),
            description: description.to_string(),
            exits: HashMap::new(),
            items: Vec::new(),
            is_exit,
            required_item,
        }
    }

    /// Adds an exit to the room
    pub fn add_exit(&mut self, direction: Direction, target_room: &str) {
        self.exits.insert(direction, target_room.to_string());
    }

    /// Adds an item to the room
    pub fn add_item(&mut self, item: &str) {
        self.items.push(item.to_string());
    }

    /// Removes an item from the room
    pub fn remove_item(&mut self, item: &str) -> bool {
        if let Some(index) = self.items.iter().position(|i| i.to_lowercase() == item.to_lowercase()) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }

    /// Gets a list of available directions
    pub fn available_exits(&self) -> Vec<&Direction> {
        self.exits.keys().collect()
    }

    /// Gets a list of available items
    pub fn available_items(&self) -> &Vec<String> {
        &self.items
    }
}

/// Creates the game world by defining rooms and their connections
pub fn create_rooms() -> HashMap<String, Room> {
    let mut rooms = HashMap::new();

    // Create rooms with descriptions
    let mut entrance = Room::new(
        "Entrance Hall",
        "You stand in the grand entrance hall of the forgotten temple. \
        Ancient symbols cover the walls, and dust particles dance in the beams of light \
        from cracks in the ceiling. The air is thick with the scent of ages past.",
        false,
        None,
    );

    let mut antechamber = Room::new(
        "Ceremonial Antechamber",
        "This room seems to have been used for pre-ritual preparations. \
        Stone benches line the walls, and faded murals depict priests donning ceremonial garb. \
        A stone altar stands in the center, its surface stained dark from ancient offerings.",
        false,
        None,
    );

    let mut treasure_room = Room::new(
        "Treasure Room",
        "Glinting gold and artifacts fill this small chamber. \
        Ceremonial masks, jeweled daggers, and strange artifacts cover every surface. \
        Despite the wealth displayed here, an ornate stone pedestal in the center stands empty, \
        with a small inscription that reads 'Place the sacred idol to reveal the path.'",
        false,
        None,
    );

    let mut idol_chamber = Room::new(
        "Guardian Chamber",
        "This circular chamber is dominated by a massive stone statue of a seated deity with many arms. \
        Its hollow eyes seem to follow your movement. At its feet lies a small golden idol, \
        gleaming despite the layer of dust covering it.",
        false,
        None,
    );

    let mut crypt = Room::new(
        "Ancient Crypt",
        "The air is stale in this dark crypt. Stone sarcophagi line the walls, \
        their carved lids depicting the deceased in repose. \
        A faded tapestry on the far wall shows a map of the stars.",
        false,
        None,
    );

    let mut temple_exit = Room::new(
        "Temple Exit",
        "Sunlight streams through a crack in the stone wall, illuminating a narrow passage. \
        This appears to be an exit from the temple, but heavy stone doors block the way. \
        There's a keyhole shaped like an idol in the center of the doors.",
        true,
        Some(String::from("golden idol")),
    );

    // Define the connections between rooms
    entrance.add_exit(Direction::North, "Ceremonial Antechamber");
    entrance.add_exit(Direction::East, "Ancient Crypt");

    antechamber.add_exit(Direction::South, "Entrance Hall");
    antechamber.add_exit(Direction::East, "Treasure Room");
    antechamber.add_exit(Direction::West, "Guardian Chamber");

    treasure_room.add_exit(Direction::West, "Ceremonial Antechamber");
    treasure_room.add_exit(Direction::North, "Temple Exit");

    idol_chamber.add_exit(Direction::East, "Ceremonial Antechamber");

    crypt.add_exit(Direction::West, "Entrance Hall");

    temple_exit.add_exit(Direction::South, "Treasure Room");

    // Place items in rooms
    idol_chamber.add_item("golden idol");
    crypt.add_item("torch");
    entrance.add_item("ancient map");
    antechamber.add_item("ceremonial dagger");

    // Add all rooms to the HashMap
    rooms.insert(entrance.name.clone(), entrance);
    rooms.insert(antechamber.name.clone(), antechamber);
    rooms.insert(treasure_room.name.clone(), treasure_room);
    rooms.insert(idol_chamber.name.clone(), idol_chamber);
    rooms.insert(crypt.name.clone(), crypt);
    rooms.insert(temple_exit.name.clone(), temple_exit);

    rooms
}
