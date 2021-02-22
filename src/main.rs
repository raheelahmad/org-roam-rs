mod publisher;
mod reader;
// use serde_lexpr::{from_str, to_string};

fn main() -> Result<(), std::io::Error> {
    let wiki = reader::read_wiki().unwrap();
    publisher::publish(wiki)?;
    Ok(())
}
