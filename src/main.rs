mod errors;
mod handler;
mod orgtag;
mod publisher;
mod reader;
mod templates;
// use serde_lexpr::{from_str, to_string};

fn main() -> Result<(), errors::ExportError> {
    let wiki = reader::read_wiki().unwrap();
    publisher::publish(wiki)?;
    Ok(())
}
