pub const CONFIG_PORT: u16 = 8083;

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
