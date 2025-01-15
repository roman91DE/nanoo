use std::fs;
use std::io;

pub fn load_file(filename: &str) -> Result<Vec<String>, io::Error> {
    let content = fs::read_to_string(filename)?;
    Ok(content.lines().map(String::from).collect())
}

pub fn save_file(filename: &str, buffer: &[String]) -> Result<(), io::Error> {
    let content = buffer.join("\n");
    fs::write(filename, content)
}
