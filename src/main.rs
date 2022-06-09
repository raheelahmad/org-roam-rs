mod errors;
mod handler;
mod orgtag;
mod publisher;
mod reader;
mod templates;

fn main() -> Result<(), errors::Error> {
    let wiki = reader::read_wiki()?;
    publisher::publish(wiki)?;
    Ok(())
}
