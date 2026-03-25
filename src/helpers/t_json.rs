use std::{fs::File, path::Path};

use serde::de::DeserializeOwned;

pub fn file_to_json<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> T {
    let data = File::open(path).unwrap();
    let reader = std::io::BufReader::new(data);
    serde_json::from_reader(reader).unwrap()
}
