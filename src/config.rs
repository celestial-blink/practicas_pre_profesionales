pub const IS_DEV: bool = match option_env!("IS_DEV") {
    Some(val) => {
        if let b"false" = val.as_bytes() {
            false
        } else {
            true
        }
    }
    None => true,
};

pub const DOMAIN: &str = match option_env!("DOMAIN") {
    Some(val) => val,
    None => "http://localhost:8083",
};
