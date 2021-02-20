use orgize::Org;
use std::io::prelude::*;
use tera::Tera;

use super::reader;

pub fn publish_file(file: &reader::OrgFile) -> Result<(), std::io::Error> {
    let path = &file.path;
    let opened_file = std::fs::read_to_string(path).expect("Should read file");
    let parsed = Org::parse(&opened_file);
    let mut writer = Vec::new();
    parsed.write_html(&mut writer).unwrap();
    let parsed_str = String::from_utf8(writer).unwrap();

    let template = main_page_template();

    let mut context = tera::Context::new();
    context.insert("page", &parsed_str);
    context.insert("tags", &file.tags.join(", "));
    let result = template.render("page.html", &context);
    println!("{}", &file.title);
    let path = "/Users/raheel/Downloads/org-roam-export/".to_string() + &file.title + ".html";
    let mut output = std::fs::File::create(path).unwrap();
    let content_bytes = result.unwrap().into_bytes();
    output.write_all(&content_bytes)?;

    Ok(())
}

fn main_page_template() -> Tera {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    tera.add_raw_template(
        "page.html",
        "<html><head> <meta charset='utf-8'/> </head><body>{{tags}}<div>{{page}}</div></body></html>",
    )
    .expect("should load raw templat");
    tera
}
