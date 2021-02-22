mod publisher;
mod reader;
// use serde_lexpr::{from_str, to_string};

fn main() -> Result<(), std::io::Error> {
    let wiki = reader::read_wiki().unwrap();
    wiki.files
        .iter()
        .for_each(|file| publisher::publish_file(file).unwrap());
    wiki.tags
        .iter()
        .for_each(|tag| publisher::publish_tag(tag).unwrap());

    Ok(())
}
