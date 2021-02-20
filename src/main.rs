mod publisher;
mod reader;
// use serde_lexpr::{from_str, to_string};

fn main() -> Result<(), std::io::Error> {
    let files = reader::read_wiki().unwrap();
    files
        .files
        .iter()
        .for_each(|file| publisher::publish_file(file).unwrap());
    Ok(())
}
