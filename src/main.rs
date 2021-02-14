use orgize::Org;
use rusqlite::{params, NO_PARAMS};
use rusqlite::{Connection, Result};
// use serde_lexpr::{from_str, to_string};

#[derive(Debug)]
struct OrgFile {
    title: String,
    path: String,
    hash: String,
    tags: Vec<String>,
    raw_meta: String,
}

impl OrgFile {
    fn add_tag(self: &mut OrgFile, tag: &str) {
        self.tags.push(String::from(tag));
    }
}

#[derive(Debug)]
struct OrgTag {
    name: String,
    file_paths: Vec<String>,
}

impl OrgTag {
    fn add_path(self: &mut OrgTag, p: &String) {
        self.file_paths.push(p.clone());
    }
}

struct Wiki {
    files: Vec<OrgFile>,
    tags: Vec<OrgTag>,
}

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
            if let Some(tag) = tags.iter_mut().find(|x| x.name == *tag_name) {
                tag.add_path(&tag_name);
            } else {
                tags.push(OrgTag {
                    name: String::from(tag_name),
                    file_paths: vec![String::from(path)],
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
        let mut path: String = row.get(1)?;
        // remove the double quotes from start/end
        path = String::from(&path[1..path.len() - 1]);

        Ok(OrgFile {
            title: row.get(0)?,
            path,
            hash: row.get(2)?,
            tags: vec![],
            raw_meta: row.get(3)?,
        })
    })?;

    let mut files: Vec<OrgFile> = vec![];
    for file in files_iter {
        match file {
            Ok(f) => files.push(f),
            Err(e) => println!("{}", e),
        }
    }
    Ok(files)
}

fn parse_file(file: &OrgFile) {
    let path = &file.path;
    let opened_file = std::fs::read_to_string(path).expect("Should read file");
    let parsed = Org::parse(&opened_file);
    let mut writer = Vec::new();
    parsed.write_html(&mut writer).unwrap();
    // println!("{:?}", String::from_utf8(writer));
}

fn main() -> Result<()> {
    let files = read_files()?;
    let result = read_tags(files)?;
    println!("{} files, {} tags", result.files.len(), result.tags.len());
    Ok(())
}
