use std::any::Any;

use crate::items::ItemTrait;

#[derive(Default, Debug)]
pub struct Inventory {
    pub items: Vec<Box<dyn Any + Send + Sync>>,
}

impl Inventory {
    pub fn query<T: ItemTrait + 'static>(&self) -> Vec<&T> {
        self.items
            .iter()
            .filter_map(|item| item.downcast_ref::<T>())
            .collect()
    }

    pub fn add<T: ItemTrait + 'static>(&mut self, item: T) {
        self.items.push(Box::new(item));
    }

    pub fn remove<T: ItemTrait + 'static + PartialEq>(&mut self, to_remove: T) -> Option<T> {
        let index = self.items.iter().position(|item| {
            if let Some(type_item) = item.downcast_ref::<T>() {
                *type_item == to_remove
            } else {
                false
            }
        })?;
        let item = self.items.remove(index);

        Some(
            *item
                .downcast::<T>()
                .expect("Failed to downcast item after check"),
        )
    }

    pub fn remove_by_id(&mut self, id: usize) -> Option<Box<dyn ItemTrait>> {
        let pos = self.items.iter().position(|item| {
            if let Some(type_item) = item.downcast_ref::<&dyn ItemTrait>() {
                type_item.id() == id
            } else {
                false
            }
        })?;

        let item = self.items.remove(pos);

        Some(
            *item
                .downcast::<Box<dyn ItemTrait>>()
                .expect("Failed to downcast item after check"),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::items::OreItem;

    use super::*;
    #[test]
    fn test_add() {
        let mut inventory = Inventory::default();
        inventory.add(OreItem::new_iron_ore(1.0, 0));
        inventory.add(OreItem::new_copper_ore(2.0, 1));
        assert!(inventory.items.len() == 2);
    }

    #[test]
    fn test_query() {
        let mut inventory = Inventory::default();
        inventory.add(OreItem::new_iron_ore(1.0, 0));
        inventory.add(OreItem::new_copper_ore(2.0, 1));

        let ores = inventory.query::<OreItem>();
        assert_eq!(ores.len(), 2);
    }

    #[test]
    fn test_remove() {
        let mut inventory = Inventory::default();
        inventory.add(OreItem::new_iron_ore(1.0, 0));
        inventory.add(OreItem::new_copper_ore(2.0, 1));

        let removed_iron_ore = inventory.remove(OreItem::new_iron_ore(1.0, 0));
        assert_eq!(removed_iron_ore.unwrap().amount(), "1");
    }

    #[test]
    fn test_remove_by_id() {
        let mut inventory = Inventory::default();
        inventory.add(OreItem::new_iron_ore(1.0, 0));
        inventory.add(OreItem::new_copper_ore(2.0, 1));

        let removed_copper_ore = inventory.remove_by_id(1);
        assert_eq!(removed_copper_ore.unwrap().amount(), "2");
    }
}
