use super::{ItemTrait, ItemType, ItemTypeWeight};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ore {
    name: &'static str,
    description: &'static str,
    hardness: f32,
    density: f32,
}

impl ItemType for Ore {
    fn name(&self) -> &'static str {
        self.name
    }

    fn description(&self) -> &'static str {
        self.description
    }

    fn weight(&self) -> ItemTypeWeight {
        ItemTypeWeight::Continuous
    }
}

const IRON_ORE: Ore = Ore {
    name: "Iron Ore",
    description: "A chunk of iron ore.",
    hardness: 3.0,
    density: 5.0,
};

const COPPER_ORE: Ore = Ore {
    name: "Copper Ore",
    description: "A chunk of copper ore.",
    hardness: 2.0,
    density: 4.0,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OreItem {
    pub ore: &'static Ore,
    pub amount: f32,
    pub purity: f32,
    pub id: usize,
}

impl OreItem {
    pub fn purify(&mut self) {
        self.amount *= self.purity;
    }

    pub fn new_iron_ore(amount: f32, id: usize) -> Self {
        Self {
            ore: &IRON_ORE,
            amount,
            purity: 0.9,
            id
        }
    }

    pub fn new_copper_ore(amount: f32, id: usize) -> Self {
        Self {
            ore: &COPPER_ORE,
            amount,
            purity: 0.8,
            id
        }
    }
}

impl ItemTrait for OreItem {
    fn get_type(&self) -> &'static dyn ItemType {
        self.ore
    }

    fn amount(&self) -> String {
        self.amount.to_string()
    }

    fn id(&self) -> usize {
        self.id
    }
}
