mod errors;
mod handler;
mod helpers;
mod orgtag;
mod publisher;
mod reader;
mod templates;
use std::path::Path;

use notify::Watcher;
use std::sync::mpsc;

fn publish_wiki() {
    if let Ok(wiki) = reader::read_wiki() {
        publisher::publish(wiki).unwrap();
    }
}

fn result_handler(res: notify::Result<notify::Event>) {
    match res {
        Ok(event) => {
            let is_file = event.paths.iter().any(|p| p.is_file());
            if !is_file {
                println!("not a file");
                return;
            }
            let valid_event =
                event.kind.is_modify() || event.kind.is_create() || event.kind.is_remove();
            if !valid_event {
                println!("not a valid event");
                return;
            }

            publish_wiki();
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}

fn main() -> Result<(), errors::Error> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::RecommendedWatcher::new(tx)?;

    watcher.watch(
        Path::new("/Users/raheel/orgs/roam"),
        notify::RecursiveMode::Recursive,
    )?;

    publish_wiki();

    for e in rx {
        result_handler(e);
    }
    Ok(())
}
