use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ItemWeight {
    Continuous(f32),
    Discrete(usize),
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

/// # Examples:
/// ```
/// use backend::items::{ Item, ItemWeight, AsAny };
///
/// #[derive(Debug)]
/// struct IronOre {
///     id: usize,
///     amount: f32,
///     purity: f32,
/// }
///
/// impl AsAny for IronOre {
///     fn as_any(&self) -> &dyn std::any::Any {
///         self
///     }
/// }
///
/// // These functions will be in a context where the type does not matter. For instance, when
/// // listing the amounts of all of the items in a list.
/// impl Item for IronOre {
///     fn type_name(&self) -> &'static str { "Iron Ore" }
///     fn type_description(&self) -> &'static str { "A rock containing iron." }
///     fn amount(&self) -> ItemWeight { ItemWeight::Continuous(self.amount) }
///     fn id(&self) -> usize { self.id }
/// }
///
/// // This will be used when the type does matter and specific operations need to be preformed on
/// // the item, specific to the item. Weapons cannot be purified, and contraining it to this type
/// // will ensure that they will only be called when valid.
/// impl IronOre {
///     pub fn purify(&mut self, amount: f32) {
///         let purity_change = (1.0 - self.purity) * amount;
///         self.purity += purity_change;
///         self.amount *= (1.0 - purity_change);
///     }
/// }
/// ```
pub trait Item: 'static + Sync + Send + Debug + AsAny{
    fn type_name(&self) -> &'static str;
    fn type_description(&self) -> &'static str;
    fn amount(&self) -> ItemWeight;
    fn id(&self) -> usize;
}
