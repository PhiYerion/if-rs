use crate::items::Item;

#[derive(Default, Debug)]
pub struct Inventory {
    pub items: Vec<Box<dyn Item>>,
}

impl Inventory {
    pub fn query<T: Item + Clone + 'static>(&self) -> Vec<T> {
        self.items
            .iter()
            .filter_map(|item| {
                let item_ref = item.as_any().downcast_ref::<T>()?;
                Some((*item_ref).clone())
            })
            .collect()
    }

    pub fn add<T: Item + 'static>(&mut self, item: T) {
        self.items.push(Box::new(item));
    }

    pub fn remove<T: Item + 'static + PartialEq + Clone>(&mut self, to_remove: T) -> Option<T> {
        let index = self.items.iter().position(|item| {
            if let Some(type_item) = item.as_any().downcast_ref::<T>() {
                *type_item == to_remove
            } else {
                false
            }
        })?;

        let dyn_removed = self.items.remove(index);
        let removed = dyn_removed.as_any().downcast_ref::<T>()?;
        Some((*removed).clone())
    }

    pub fn remove_by_id(&mut self, id: usize) -> Option<Box<dyn Item>> {
        let pos = self.items.iter().position(|item| item.id() == id)?;

        let item = self.items.remove(pos);

        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use crate::items::ore::{CopperOre, IronOre, Ore};
    use crate::items::ItemWeight;

    use super::*;
    #[test]
    fn test_add() {
        let mut inventory = Inventory::default();
        inventory.add(Ore::<IronOre>::new(1.0, 1.0, 1));
        inventory.add(Ore::<CopperOre>::new(2.0, 1.0, 1));
        assert!(inventory.items.len() == 2);
    }

    #[test]
    fn test_query() {
        let mut inventory = Inventory::default();
        inventory.add(Ore::<IronOre>::new(1.0, 1.0, 0));
        inventory.add(Ore::<CopperOre>::new(2.0, 1.0, 1));
        inventory.add(Ore::<CopperOre>::new(3.0, 1.0, 2));

        let iron_ores = inventory.query::<Ore<IronOre>>();
        let copper_ores = inventory.query::<Ore<CopperOre>>();
        assert_eq!(iron_ores.len(), 1);
        assert_eq!(copper_ores.len(), 2);
    }

    #[test]
    fn test_remove() {
        let mut inventory = Inventory::default();
        inventory.add(Ore::<IronOre>::new(1.0, 1.0, 0));
        let ore_to_check = Ore::<CopperOre>::new(2.0, 1.0, 1);
        inventory.add(ore_to_check.clone());
        inventory.add(Ore::<CopperOre>::new(3.0, 1.0, 2));

        let removed_iron_ore = inventory.remove(ore_to_check.clone());
        assert_eq!(removed_iron_ore.unwrap(), ore_to_check);
    }

    #[test]
    fn test_remove_by_id() {
        let mut inventory = Inventory::default();
        inventory.add(Ore::<IronOre>::new(1.0, 1.0, 0));
        inventory.add(Ore::<CopperOre>::new(2.0, 1.0, 1));
        inventory.add(Ore::<CopperOre>::new(3.0, 1.0, 2));

        let removed_copper_ore = inventory.remove_by_id(1);
        assert_eq!(
            removed_copper_ore.unwrap().amount(),
            ItemWeight::Continuous(2.0)
        );
    }
}
