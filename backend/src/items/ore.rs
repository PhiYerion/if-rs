use bevy::asset::Handle;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::component::Component;
use bevy::scene::Scene;
use bevy::transform::components::Transform;

use super::{Item, SpecificItem};
use crate::anyify;
use crate::items::ItemWeight;
use bevy_xpbd_3d::components::RigidBody;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Bundle)]
pub struct OreBundle<T: OreType> {
    pub ore: Ore<T>,
    pub rigid_body: RigidBody,
    pub model: Handle<Scene>,
    pub transform: Transform,
}

/// Denotes the type of ore the [Ore] struct is representing
pub trait OreType: Debug + Send + Sync + PartialEq + Clone + Copy + PartialOrd + 'static {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Component)]
pub struct Ore<T: OreType> {
    ore_type: PhantomData<T>,
    pub purity: f32,
    pub amount: f32,
    pub id: usize,
}

#[rustfmt::skip]
impl<T: OreType> Ore<T> {
    pub fn amount(&self) -> super::ItemWeight { ItemWeight::Continuous(self.amount) }
    pub fn id(&self) -> usize { self.id }

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

impl SpecificItem for Ore<CopperOre> {
    type B = OreBundle<CopperOre>;
    type M = f32;

    fn split(&mut self, amount: f32) -> Option<Self> {
        if amount > self.amount {
            return None;
        }

        self.amount -= amount;
        Some(Ore {
            ore_type: PhantomData,
            purity: self.purity,
            amount,
            id: self.id,
        })
    }
}

anyify!(Ore<CopperOre>);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct IronOre;
impl OreType for IronOre {}

#[rustfmt::skip]
impl Item for Ore<IronOre> {
    fn type_name(&self) -> &'static str { "Copper Ore" }
    fn type_description(&self) -> &'static str { "A rock containing copper." }
    fn amount(&self) -> ItemWeight { ItemWeight::Continuous(self.amount) }
    fn id(&self) -> usize { self.id() }
}

impl SpecificItem for Ore<IronOre> {
    type B = OreBundle<IronOre>;
    type M = f32;

    fn split(&mut self, amount: f32) -> Option<Self> {
        if amount > self.amount {
            return None;
        }

        self.amount -= amount;
        Some(Ore {
            ore_type: PhantomData,
            purity: self.purity,
            amount,
            id: self.id,
        })
    }
}

anyify!(Ore<IronOre>);
