use std::collections::HashMap;
use std::time::SystemTime;

use chrono::prelude::*;
use serde::Serialize;

use crate::helpers::{self, trim_start_end_char};

#[derive(Debug, Serialize, Clone)]
pub struct OrgFile {
    pub title: String,
    pub path: String,
    pub id: String,
    pub tags: Vec<String>,
    pub raw_file: String,
    pub referenced_file_paths: Vec<String>,
    modified: SystemTime,
}

impl OrgFile {
    pub fn add_tag(self: &mut OrgFile, tag: &str) {
        self.tags.push(tag.to_string());
    }

    pub fn new(title: String, path: String, id: String, tags: Vec<String>) -> OrgFile {
        let path = helpers::trim_start_end_char(&path);
        let title = helpers::trim_start_end_char(&title);
        let id = helpers::trim_start_end_char(&id);

        let err_str = format!("Should read {}", path);
        let raw_file = std::fs::read_to_string(&path).expect(&err_str);

        let referenced_file_paths = OrgFile::referenced_file_paths(&path);
        let modified = OrgFile::last_modified_date(&path);
        OrgFile {
            title,
            path,
            id,
            raw_file,
            tags,
            referenced_file_paths,
            modified,
        }
    }
}

impl OrgFile {
    pub fn modified_days_ago(self: &OrgFile) -> i32 {
        let chrono_modified = chrono::DateTime::<Utc>::from(self.modified);
        let chrono_now = Utc::now();

        chrono_now.num_days_from_ce() - chrono_modified.num_days_from_ce()
    }
}

#[derive(Debug)]
pub struct OrgTag {
    pub name: String,
    pub files: Vec<String>,
}

impl OrgTag {
    pub fn add_path(self: &mut OrgTag, p: String) {
        self.files.push(p);
    }
}

#[derive(Default)]
pub struct Wiki {
    pub files: Vec<OrgFile>,
    pub tags: Vec<OrgTag>,
}

impl Wiki {
    pub fn base_export_path() -> String {
        String::from("/Users/raheel/Projects/served/wiki/")
    }

    pub fn base_org_roam_path() -> String {
        String::from("/Users/raheel/orgs/roam")
    }
}

impl OrgTag {
    pub fn output_file(self: &OrgTag) -> Result<std::fs::File, std::io::Error> {
        let path = Wiki::base_export_path() + "tag-" + &self.name + ".html";
        let file = std::fs::File::create(path)?;
        Ok(file)
    }
}

fn is_orgfile_path(file: &str) -> bool {
    file.starts_with("file:") && file.ends_with(".org")
}

impl OrgFile {
    fn referenced_file_paths(file: &str) -> Vec<String> {
        let opened_file = std::fs::read_to_string(file).expect("Should read file");
        let org = orgize::Org::parse(&opened_file);
        // let mut printed_header = false;
        let mut paths = vec![];
        for p in org.iter() {
            if let orgize::Event::Start(orgize::Element::Link(link)) = p {
                if is_orgfile_path(&link.path) {
                    paths.push(link.path.strip_prefix("file:").unwrap().to_string());
                }
            }
        }
        paths
    }

    fn last_modified_date(file: &str) -> SystemTime {
        let file = std::fs::metadata(file).expect("should read file metadata");
        file.modified().unwrap()
    }
}

#[derive(Serialize)]
pub struct FilesForTag {
    pub tag_name: String,
    pub files: Vec<OrgFile>,
}

impl FilesForTag {
    pub fn build(wiki: &Wiki) -> Vec<FilesForTag> {
        let mut hash: HashMap<String, Vec<OrgFile>> = HashMap::new();
        for file in &wiki.files {
            for tag in &file.tags {
                if let Some(files) = hash.get_mut(tag) {
                    files.push(file.clone());
                } else {
                    hash.insert(tag.clone(), vec![file.clone()]);
                }
            }
        }
        let mut result: Vec<FilesForTag> = vec![];
        for (tag_name, files_for_tag) in hash {
            result.push(FilesForTag {
                tag_name,
                files: files_for_tag,
            });
        }
        result
    }
}

#[derive(Debug, Serialize)]
pub struct FilesSorted {
    pub files: Vec<OrgFile>,
}

impl FilesSorted {
    pub fn build(wiki: &Wiki) -> FilesSorted {
        let mut files = wiki.files.clone();
        files.sort_by(|a, b| {
            a.modified_days_ago()
                .partial_cmp(&b.modified_days_ago())
                .unwrap()
        });

        FilesSorted { files }
    }
}

#[derive(Serialize)]
pub struct FilesByWeeksAway {
    pub files: Vec<OrgFile>,
    pub weeks_away: i32,
}

impl FilesByWeeksAway {
    pub fn build(wiki: &Wiki) -> Vec<FilesByWeeksAway> {
        let mut hash: HashMap<i32, Vec<OrgFile>> = HashMap::new();
        for file in &wiki.files {
            let days_ago = file.modified_days_ago();
            if let Some(files) = hash.get_mut(&days_ago) {
                files.push(file.clone());
            } else {
                hash.insert(days_ago, vec![file.clone()]);
            }
        }
        let mut result: Vec<FilesByWeeksAway> = vec![];
        for (key, value) in hash {
            result.push(FilesByWeeksAway {
                files: value,
                weeks_away: key - 1, // start at 0
            });
        }
        result.sort_by(|a, b| a.weeks_away.partial_cmp(&b.weeks_away).unwrap());
        result
    }
}
