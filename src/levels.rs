pub struct Level {
    pub name: String,
    pub splits: i8,
    pub is_menu: bool,
}

impl Level {
    fn new(name: &str, splits: i8, is_menu: bool) -> Self {
        Self {
            name: name.to_string(),
            splits,
            is_menu,
        }
    }
}

pub(crate) fn get_levels() -> Vec<Level> {
    vec![
        // ------------------------------------------------------- SANDLAKE
        Level::new("Sandlake - 1", 1, false),
        Level::new("Sandlake - 2", 1, false),
        Level::new("Robot - Bliss", 2, false),
        Level::new("Sandlake - 3", 1, false),
        Level::new("Sandlake - 4", 1, false),
        Level::new("Sandlake - 5", 1, false),
        Level::new("Robot - Gino", 2, false),
        Level::new("Sandlake - 5b", 1, false),
        Level::new("Sandlake - 6", 1, false),
        Level::new("Sandlake - 7", 1, false),
        Level::new("Sandlake - 8", 1, false),
        Level::new("Sandlake - 9", 1, false),
        // ------------------------------------------------------- WHISPERING CAVERN
        Level::new("Wispering Cavern - Menu", 2, true),
        Level::new("Wispering Cavern - 1", 1, false),
        Level::new("Wispering Cavern - 2", 1, false),
        Level::new("Wispering Cavern - 3", 1, false),
        Level::new("Wispering Cavern - 4", 1, false),
        Level::new("Wispering Cavern - 5", 1, false),
        Level::new("Wispering Cavern - 6", 1, false),
        Level::new("Wispering Cavern - 7", 1, false),
        Level::new("Wispering Cavern - 8", 1, false),
        Level::new("Wispering Cavern - 9", 1, false),
        // ------------------------------------------------------- MAGNETIC CANYON
        Level::new("Magnetic Canyon - Menu", 2, true),
        Level::new("Magnetic Canyon - 1", 1, false),
        Level::new("Magnetic Canyon - 2", 1, false),
        Level::new("Magnetic Canyon - 3", 1, false),
        Level::new("Magnetic Canyon - 4", 1, false),
        Level::new("Magnetic Canyon - 5", 1, false),
        Level::new("Magnetic Canyon - 6", 1, false),
        Level::new("Magnetic Canyon - 7", 1, false),
        Level::new("Magnetic Canyon - 8", 1, false),
        Level::new("Magnetic Canyon - 9", 1, false),
        // ------------------------------------------------------- ANCESTRAL FOREST
        Level::new("Ancestral Forest - Menu", 2, true),
        Level::new("Ancestral Forest - 1", 1, false),
        Level::new("Ancestral Forest - 2", 1, false),
        Level::new("Ancestral Forest - 3", 1, false),
        Level::new("Ancestral Forest - 4", 1, false),
        Level::new("Ancestral Forest - 5", 1, false),
        Level::new("Ancestral Forest - 6", 1, false),
        Level::new("Ancestral Forest - 7", 1, false),
        Level::new("Ancestral Forest - 8", 1, false),
        Level::new("Ancestral Forest - 9", 1, false),
        // ------------------------------------------------------- SNOWY GRAVEYARD
        Level::new("Snowy Graveyard - Menu", 1, true),
        Level::new("Snowy Graveyard - 1", 1, false),
        Level::new("Snowy Graveyard - 2", 1, false),
        Level::new("Snowy Graveyard - 3", 1, false),
        Level::new("Snowy Graveyard - 4", 1, false),
        Level::new("Snowy Graveyard - 5", 1, false),
        Level::new("Snowy Graveyard - 6", 1, false),
        Level::new("Snowy Graveyard - 7", 1, false),
        Level::new("Snowy Graveyard - 8", 1, false),
        Level::new("Snowy Graveyard - BOSS", 1, false),
        Level::new("Robot - Yu", 1, false),
        // ------------------------------------------------------- MILITARY BASE
        Level::new("Military Base - Menu", 1, true),
        Level::new("Military Base - 1", 1, false),
        Level::new("Military Base - 2", 1, false),
        Level::new("Military Base - 3", 1, false),
        Level::new("Military Base - 4", 1, false),
        Level::new("Military Base - 5", 1, false),
        Level::new("Military Base - 6", 1, false),
        Level::new("Military Base - 7", 1, false),
        Level::new("Military Base - 8", 1, false),
        Level::new("Military Base - BOSS", 1, false),
        // ------------------------------------------------------- SPACE ELEVATOR
        Level::new("Space Elevator - Menu", 1, true),
        Level::new("Space Elevator - 1", 1, false),
        Level::new("Space Elevator - 2", 1, false),
        Level::new("Space Elevator - 3", 1, false),
        Level::new("Space Elevator - 4", 1, false),
        Level::new("Space Elevator - 5", 1, false),
        Level::new("Space Elevator - 6", 1, false),
        Level::new("Space Elevator - 7", 1, false),
        Level::new("Space Elevator - 8", 1, false),
        Level::new("Space Elevator - BOSS", 1, false),
        Level::new("Credits", 1, false),
    ]
}
