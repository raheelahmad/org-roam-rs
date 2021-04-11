use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct OrgFile {
    pub title: String,
    pub path: String,
    hash: String,
    pub tags: Vec<String>,
    pub raw_file: String,
    raw_meta: String,
    pub referenced_file_paths: Vec<String>,
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

        let err_str = format!("Should read {}", path);
        let raw_file = std::fs::read_to_string(&path).expect(&err_str);

        let referenced_file_paths = OrgFile::referenced_file_paths(&path);
        OrgFile {
            title,
            path,
            hash,
            raw_file,
            tags,
            raw_meta,
            referenced_file_paths,
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
            if let orgize::Event::Start(element) = p {
                if let orgize::Element::Link(link) = element {
                    if is_orgfile_path(&link.path) {
                        // if !printed_header {
                        //     println!("\n\nFor {}", file);
                        //     printed_header = true;
                        // }
                        paths.push(link.path.strip_prefix("file:").unwrap().to_string());
                    }
                }
            }
        }
        paths
    }
}
