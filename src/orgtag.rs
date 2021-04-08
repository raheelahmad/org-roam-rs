use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct OrgFile {
    pub title: String,
    pub path: String,
    hash: String,
    pub tags: Vec<String>,
    raw_meta: String,
}

impl OrgFile {
    pub fn add_tag(self: &mut OrgFile, tag: &str) {
        self.tags.push(String::from(tag));
    }

    pub fn new(
        title: String,
        path: String,
        hash: String,
        tags: Vec<String>,
        raw_meta: String,
    ) -> OrgFile {
        let mut path: String = path;
        // remove the double quotes from start/end
        path = String::from(&path[1..path.len() - 1]);
        let mut title: String = title;
        // remove the double quotes from start/end
        title = String::from(&title[1..title.len() - 1]);
        OrgFile {
            title,
            path,
            hash,
            tags,
            raw_meta,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OrgTagFile {
    pub title: String,
}

#[derive(Debug)]
pub struct OrgTag {
    pub name: String,
    pub files: Vec<OrgTagFile>,
}

impl OrgTag {
    pub fn add_path(self: &mut OrgTag, p: OrgTagFile) {
        self.files.push(p);
    }
}

#[derive(Default)]
pub struct Wiki {
    pub files: Vec<OrgFile>,
    pub tags: Vec<OrgTag>,
}

impl Wiki {
    pub fn base_path() -> String {
        String::from("/Users/raheel/Downloads/org-roam-export/")
    }
}

impl OrgTag {
    pub fn output_file(self: &OrgTag) -> Result<std::fs::File, std::io::Error> {
        let path = Wiki::base_path() + "tag-" + &self.name + ".html";
        let file = std::fs::File::create(path)?;
        Ok(file)
    }
}
