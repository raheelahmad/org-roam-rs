mod errors;
mod handler;
mod helpers;
mod orgtag;
mod publisher;
mod reader;
mod templates;
use std::path::Path;

use notify::Watcher;

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
    let mut watcher = notify::recommended_watcher(result_handler)?;

    // let watcher = notify::watcher(std::time::Duration::from_secs(8)).unwrap();
    // notify::watcher(, delay)
    watcher.watch(
        Path::new("/Users/raheel/orgs/roam"),
        notify::RecursiveMode::Recursive,
    )?;
    publish_wiki();
    loop {}
    Ok(())
}
