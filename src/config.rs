pub const IS_DEV: bool = match option_env!("IS_DEV") {
    Some(val) => {
        if let b"true" = val.as_bytes() {
            true
        } else {
            false
        }
    },
    None => false,
};
