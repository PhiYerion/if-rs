use super::{AsAny, Item};
use crate::items::ItemWeight;
use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;

/// Denotes the type of ore the [Ore] struct is representing
pub trait OreType: Debug + Send + Sync + PartialEq + Clone + Copy + PartialOrd {}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Ore<T: OreType> {
    ore_type: PhantomData<T>,
    pub purity: f32,
    pub amount: f32,
    pub id: usize,
}

#[rustfmt::skip]
impl<T: OreType + 'static> Ore<T> {
    fn amount(&self) -> super::ItemWeight { ItemWeight::Continuous(self.amount) }
    fn id(&self) -> usize { self.id }
}

impl<T: OreType> Ore<T> {
    pub fn purify(&mut self, percent_change: f32) -> f32 {
        let clamped_change = percent_change.clamp(self.purity, 1.0 - self.purity);
        self.purity += clamped_change;
        self.amount *= 1.0 - clamped_change;

        clamped_change
    }

    pub fn new(amount: f32, purity: f32, id: usize) -> Self {
        Ore {
            ore_type: PhantomData,
            purity,
            amount,
            id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CopperOre;
impl OreType for CopperOre {}

#[rustfmt::skip]
impl Item for Ore<CopperOre> {
    fn type_name(&self) -> &'static str { "Copper Ore" }
    fn type_description(&self) -> &'static str { "A rock containing copper." }
    fn amount(&self) -> ItemWeight { self.amount() }
    fn id(&self) -> usize { self.id() }
}

impl AsAny for Ore<CopperOre> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct IronOre;
impl OreType for IronOre {}

#[rustfmt::skip]
impl Item for Ore<IronOre> {
    fn type_name(&self) -> &'static str { "Iron Ore" }
    fn type_description(&self) -> &'static str { "A rock containing copper." }
    fn amount(&self) -> ItemWeight { self.amount() }
    fn id(&self) -> usize { self.id() }
}

impl AsAny for Ore<IronOre> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
