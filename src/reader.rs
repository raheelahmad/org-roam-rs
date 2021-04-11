use rusqlite::{params, NO_PARAMS};
use rusqlite::{Connection, Result};

use super::orgtag::{OrgFile, OrgTag, OrgTagFile, Wiki};

fn read_tags(mut files: Vec<OrgFile>) -> Result<Wiki> {
    let conn = Connection::open("/Users/raheel/.emacs.d/org-roam.db")?;
    let mut stmt = conn.prepare("SELECT file, tags from tags WHERE file IS NOT NULL;")?;

    let mut tags: Vec<OrgTag> = vec![];

    #[derive(Debug)]
    struct TagResult {
        path: String,
        tags: Vec<String>,
    }

    let tag_results = stmt.query_map(params![], |row| {
        let mut path: String = row.get(0)?;
        path = String::from(&path[1..path.len() - 1]);
        let tag_name_str: String = row.get(1)?;
        let tags_names: Vec<String> = serde_lexpr::from_str(&tag_name_str).unwrap();

        Ok(TagResult {
            path,
            tags: tags_names,
        })
    })?;

    for tag_result in tag_results {
        let result = tag_result?;
        let path = &result.path;
        let file = files
            .iter_mut()
            .find(|x| x.path == *path)
            .expect("Something went wrong");
        for tag_name in &result.tags {
            file.add_tag(&tag_name);
            let tag_file = OrgTagFile {
                title: file.title.clone(),
            };
            if let Some(tag) = tags.iter_mut().find(|x| x.name == *tag_name) {
                tag.add_path(tag_file);
            } else {
                tags.push(OrgTag {
                    name: String::from(tag_name),
                    files: vec![tag_file],
                });
            }
        }
    }

    Ok(Wiki { files, tags })
}

fn read_files() -> Result<Vec<OrgFile>> {
    let conn = Connection::open("/Users/raheel/.emacs.d/org-roam.db")?;
    let mut stmt = conn.prepare("SELECT t1.title, f1.file, f1.hash, f1.meta FROM titles t1, files f1 where t1.file == f1.file")?;

    let files_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(OrgFile::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            vec![],
            row.get(3)?,
        ))
    })?;

    let mut files: Vec<OrgFile> = vec![];
    for file in files_iter {
        match file {
            Ok(f) => files.push(f),
            Err(e) => println!("{}", e),
        }
    }
    files.sort_by_key(|f| f.elapsed());

    Ok(files)
}

trait Elapsed {
    fn elapsed(&self) -> std::time::Duration;
}

impl Elapsed for OrgFile {
    fn elapsed(&self) -> std::time::Duration {
        let file = std::fs::File::open(&self.path).unwrap();
        file.metadata()
            .unwrap()
            .modified()
            .unwrap()
            .elapsed()
            .unwrap()
    }
}

pub fn read_wiki() -> Result<Wiki> {
    let files = read_files()?;
    let wiki = read_tags(files)?;
    Ok(wiki)
}
