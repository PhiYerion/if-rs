pub enum ItemTypeWeight {
    Continuous,
    Discrete(f32),
}

pub trait ItemType: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn weight(&self) -> ItemTypeWeight;
}
