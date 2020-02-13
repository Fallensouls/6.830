use std::any::Any;

pub trait Downcast {
    fn as_any(&self) -> &dyn Any;
}

#[macro_export]
macro_rules! downcast {
    ($value:expr, $type:ty) => {
        *$value.as_any().downcast_ref::<$type>().unwrap()
    };
}
