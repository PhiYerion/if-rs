use std::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[macro_export]
macro_rules! anyify {
    ($($t:ty),*) => {
        $(
            impl $crate::as_any::AsAny for $t {
                fn as_any(&self) -> &dyn std::any::Any {
                    self
                }
                fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                    self
                }
            }
        )*
    };
}
