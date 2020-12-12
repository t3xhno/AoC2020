use std::fs;

pub fn get_input_data (file_name: &str) -> String {
    fs::read_to_string(file_name)
        .expect("Something went wrong...")
}