use crate::as_any::AsAny;
use crate::items::{Item, SpecificItem};
use std::any::Any;
use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct Inventory {
    items: Vec<Box<dyn ItemVecTrait>>,
}

pub trait ItemVecTrait: Any + Debug + AsAny {
    fn as_generic(&self) -> Vec<&dyn Item>;
    fn as_generic_mut(&mut self) -> Vec<&mut dyn Item>;
}

impl<T: SpecificItem> ItemVecTrait for Vec<T> {
    fn as_generic(&self) -> Vec<&dyn Item> {
        self.iter().map(|item| item as &dyn Item).collect()
    }

    fn as_generic_mut(&mut self) -> Vec<&mut dyn Item> {
        self.iter_mut().map(|item| item as &mut dyn Item).collect()
    }
}

impl<T: SpecificItem> AsAny for Vec<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Inventory {
    #[rustfmt::skip]
    pub fn query<T: SpecificItem>(&self) -> Option<&Vec<T>> {
        let vec = self.items
            .iter()
            .find(|vec| vec.as_any().is::<Vec<T>>())?
            .as_any().downcast_ref::<Vec<T>>().unwrap();

        Some(vec)
    }

    #[rustfmt::skip]
    pub fn query_mut<T: SpecificItem>(&mut self) -> Option<&mut Vec<T>> {
        let vec = self.items
            .iter_mut()
            .find(|vec| vec.as_any().is::<Vec<T>>())?
            .as_any_mut().downcast_mut::<Vec<T>>()
            .unwrap();

        Some(vec)
    }

    pub fn get_all(&self) -> Vec<&dyn Item> {
        self.items.iter().fold(Vec::new(), |mut acc, vec| {
            acc.append(&mut vec.as_generic());
            acc
        })
    }

    pub fn add<T: SpecificItem>(&mut self, item: T) {
        match self.query_mut::<T>() {
            Some(vec) => vec.push(item),
            None => {
                let new_vec = Box::new(vec![item]);
                self.items.push(new_vec);
            }
        }
    }

    pub fn remove<T: SpecificItem>(&mut self, to_remove: T) -> Option<T> {
        let vec = self.query_mut::<T>()?;
        let index = vec.iter().position(|item| *item == to_remove)?;
        Some(vec.remove(index))
    }

    pub fn remove_by_id<T: SpecificItem>(&mut self, id: usize) -> Option<T> {
        let vec = self.query_mut::<T>()?;
        let index = vec.iter().position(|item| item.id() == id)?;
        Some(vec.remove(index))
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

        let iron_ores = inventory
            .query::<Ore<IronOre>>()
            .expect("Did not find Iron Ores");
        let copper_ores = inventory
            .query::<Ore<CopperOre>>()
            .expect("Did not find Copper Ores");
        assert_eq!(iron_ores.len(), 1);
        assert_eq!(copper_ores.len(), 2);
    }

    #[test]
    fn test_remove() {
        let mut inventory = Inventory::default();
        inventory.add(Ore::<IronOre>::new(1.0, 1.0, 0));
        let ore_to_check = Ore::<CopperOre>::new(2.0, 1.0, 1);
        inventory.add(ore_to_check);
        inventory.add(Ore::<CopperOre>::new(3.0, 1.0, 2));

        let removed_iron_ore = inventory.remove(ore_to_check);
        assert_eq!(removed_iron_ore.unwrap(), ore_to_check);
    }

    #[test]
    fn test_remove_by_id() {
        let mut inventory = Inventory::default();
        inventory.add(Ore::<IronOre>::new(1.0, 1.0, 0));
        inventory.add(Ore::<CopperOre>::new(2.0, 1.0, 1));
        inventory.add(Ore::<CopperOre>::new(3.0, 1.0, 2));

        let removed_copper_ore = inventory.remove_by_id::<Ore<CopperOre>>(1);
        assert_eq!(
            removed_copper_ore.unwrap().amount(),
            ItemWeight::Continuous(2.0)
        );
    }

    #[test]
    fn get_all() {
        let mut inventory = Inventory::default();
        inventory.add(Ore::<IronOre>::new(1.0, 1.0, 0));
        inventory.add(Ore::<CopperOre>::new(2.0, 1.0, 1));
        inventory.add(Ore::<CopperOre>::new(3.0, 1.0, 2));

        let all = inventory.get_all();
        assert_eq!(all.len(), 3);
    }
}
