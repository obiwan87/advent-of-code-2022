use std::fs;

pub(crate) fn load_data_to_str(path: &str) -> String {
    return fs::read_to_string(path).expect("Could not read from file");
}