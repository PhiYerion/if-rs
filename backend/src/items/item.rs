use std::fmt::Debug;

use super::ItemType;

pub trait ItemTrait: 'static + Sync + Send + Debug {
    fn amount(&self) -> String;
    fn get_type(&self) -> &'static dyn ItemType;
    fn id(&self) -> usize;
}
