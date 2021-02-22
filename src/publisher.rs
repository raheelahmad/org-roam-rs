use orgize::Org;
use reader::OrgTag;
use std::io::prelude::*;
use tera::Tera;

use super::reader;

fn base_path() -> String {
    String::from("/Users/raheel/Downloads/org-roam-export/")
}

impl OrgTag {
    fn output_file(self: &OrgTag) -> Result<std::fs::File, std::io::Error> {
        let path = base_path() + "tag-" + &self.name + ".html";
        let file = std::fs::File::create(path)?;
        Ok(file)
    }
}

pub fn publish(wiki: reader::Wiki) -> Result<(), std::io::Error> {
    wiki.files.iter().try_for_each(|file| publish_file(file))?;
    wiki.tags.iter().try_for_each(|tag| publish_tag(tag))?;
    publish_index(&wiki)?;
    Ok(())
}

fn publish_index(wiki: &reader::Wiki) -> Result<(), std::io::Error> {
    let tempalte = index_template();
    let mut context = tera::Context::new();
    context.insert("pages", &wiki.files);
    let render_result = tempalte.render("index.html", &context).unwrap();
    let content_bytes = render_result.into_bytes();
    let path = base_path() + "index.html";
    let mut output = std::fs::File::create(path).unwrap();
    output.write_all(&content_bytes)?;

    Ok(())
}

fn publish_tag(tag: &reader::OrgTag) -> Result<(), std::io::Error> {
    let tempalte = tag_page_template();
    let mut context = tera::Context::new();
    context.insert("tag_name", &tag.name);
    context.insert("pages", &tag.files);
    let render_result = tempalte.render("tag.html", &context).unwrap();
    let content_bytes = render_result.into_bytes();
    let mut output = tag.output_file()?;
    output.write_all(&content_bytes)?;
    println!("{}", &tag.name);

    Ok(())
}

fn publish_file(file: &reader::OrgFile) -> Result<(), std::io::Error> {
    let path = &file.path;
    let opened_file = std::fs::read_to_string(path).expect("Should read file");
    let parsed = Org::parse(&opened_file);
    let mut writer = Vec::new();
    parsed.write_html(&mut writer).unwrap();
    let parsed_str = String::from_utf8(writer).unwrap();

    let template = page_template();

    let mut context = tera::Context::new();
    context.insert("page", &parsed_str);
    context.insert("tags", &file.tags);
    let result = template.render("page.html", &context);
    let path = base_path() + &file.title + ".html";
    let mut output = std::fs::File::create(path).unwrap();
    let content_bytes = result.unwrap().into_bytes();
    output.write_all(&content_bytes)?;

    Ok(())
}

fn tag_page_template() -> Tera {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    tera.add_raw_template(
        "tag.html",
        "
<html><head>
<meta charset='utf-8'/> </head>
<body>
<div>
All pages for <strong>{{tag_name}}</strong>

<ul>
{% for page in pages %}
<li>
  <a href='{{page.title}}.html'>{{page.title}}</a>
</li>
{% endfor %}
</ul>
</div>

</body></html>
",
    )
    .expect("should load raw templat");
    tera
}

fn index_template() -> Tera {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    tera.add_raw_template(
        "index.html",
        "<html><head> <meta charset='utf-8'/> </head>
<body>
<ul>
{% for page in pages %}
<li>
  <a href='{{page.title}}.html'>{{page.title}}</a>
</li>
{% endfor %}
</ul>
</body>
</html>",
    )
    .expect("should load raw templat");
    tera
}

fn page_template() -> Tera {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    tera.add_raw_template(
        "page.html",
        "<html><head> <meta charset='utf-8'/> </head>
<body>
	{% for tag in tags %}
    <a href='tag-{{tag }}.html'>{{ tag }}</a>
	{% endfor %}
	<div>{{page}}</div>
</body>
</html>",
    )
    .expect("should load raw templat");
    tera
}
