use rusqlite::{params, NO_PARAMS};
use rusqlite::{Connection, Result};

use super::orgtag::{OrgFile, OrgTag, Wiki};

fn read_tags(mut files: Vec<OrgFile>, conn: &Connection) -> Result<Wiki> {
    let mut stmt = conn.prepare(
        "SELECT n.file, t.tag from tags t, nodes n WHERE n.file IS NOT NULL AND n.id = t.node_id;",
    )?;

    let mut tags: Vec<OrgTag> = vec![];

    struct TagRow {
        path: String,
        tag: String,
    }
    let tag_results = stmt.query_map(params![], |row| {
        let mut path: String = row.get(0)?;
        path = crate::helpers::trim_start_end_char(&path);

        Ok(TagRow {
            path,
            tag: row.get(1)?,
        })
    })?;

    // add tags to each file
    // construct OrgTag that lists all files

    for tag_result in tag_results {
        let result = tag_result?;
        let path = &result.path;
        let file = files
            .iter_mut()
            .find(|x| x.path == *path)
            .expect("Something went wrong");
        file.add_tag(&result.tag);

        let tag_file = file.title.clone();
        if let Some(tag) = tags.iter_mut().find(|x| x.name == result.tag) {
            tag.add_path(tag_file);
        } else {
            tags.push(OrgTag {
                name: String::from(result.tag),
                files: vec![tag_file],
            });
        }
    }
    Ok(Wiki { files, tags })
}

fn read_files(conn: &Connection) -> Result<Vec<OrgFile>> {
    let mut stmt = conn.prepare(
        "SELECT t1.title, f1.file, t1.id FROM nodes t1, files f1 where t1.file == f1.file",
    )?;

    let mut files = stmt
        .query_map(NO_PARAMS, |row| {
            Ok(OrgFile::new(row.get(0)?, row.get(1)?, row.get(2)?, vec![]))
        })?
        .filter_map(|f| f.ok())
        .filter(|f| f.title != "Recent changes")
        .collect::<Vec<OrgFile>>();

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
    let conn = Connection::open("/Users/raheel/.emacs.d/org-roam.db")?;
    let files = read_files(&conn)?;
    let wiki = read_tags(files, &conn)?;
    Ok(wiki)
}
