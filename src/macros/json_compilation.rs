#[macro_export]
macro_rules! load_json {
    ($path:literal, $type:ty) => {
        serde_json::from_slice::<$type>(include_bytes!($path)).unwrap()
    };
}
