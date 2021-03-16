use orgize::{export::HtmlHandler, Org};
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

pub fn publish(wiki: reader::Wiki) -> Result<(), ExportError> {
    wiki.files.iter().try_for_each(|file| publish_file(file))?;
    wiki.tags.iter().try_for_each(|tag| publish_tag(tag))?;
    copy_images_deux().expect("Should copy successfully");
    publish_index(&wiki)?;
    Ok(())
}

fn copy_images_deux() -> Result<(), fs_extra::error::Error> {
    let from = std::path::Path::new("/Users/raheel/orgs/roam/images");
    let to = std::path::Path::new("/Users/raheel/Downloads/org-roam-export");
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    fs_extra::dir::copy(from, to, &options)?;
    Ok(())
}

fn publish_index(wiki: &reader::Wiki) -> Result<(), std::io::Error> {
    let template = index_template();
    let mut context = tera::Context::new();
    context.insert("pages", &wiki.files);
    let render_result = template.render("index.html", &context).unwrap();
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

    Ok(())
}

#[derive(Debug)]
pub enum ExportError {
    Random(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::Random(reason) => write!(f, "{}", (reason)),
        }
    }
}
impl From<std::io::Error> for ExportError {
    fn from(error: std::io::Error) -> Self {
        ExportError::Random(error.to_string())
    }
}

#[derive(Default)]
struct CustomHTMLHandler(orgize::export::DefaultHtmlHandler);

impl HtmlHandler<ExportError> for CustomHTMLHandler {
    fn start<W: Write>(&mut self, mut w: W, element: &orgize::Element) -> Result<(), ExportError> {
        if let orgize::Element::Link(link) = element {
            if link.path.ends_with("png") {
                let path = &link.path;
                // let filename = path.split('/').last().unwrap().replace(' ', "%20");
                let filename = path
                    .strip_prefix("file:images/")
                    .or_else(|| path.strip_prefix("file:/images/"))
                    .unwrap();
                // .unwrap_or(path.strip_prefix("file:/images/").unwrap());

                write!(w, "<img src='/images/{}'/>", filename).unwrap();
            } else {
                self.0.start(w, element)?;
            }
        } else {
            self.0.start(w, element)?;
        }
        Ok(())
    }

    fn end<W: Write>(&mut self, w: W, element: &orgize::Element) -> Result<(), ExportError> {
        self.0.end(w, element)?;
        Ok(())
    }
}

fn publish_file(file: &reader::OrgFile) -> Result<(), ExportError> {
    let path = &file.path;
    let opened_file = std::fs::read_to_string(path).expect("Should read file");
    let parsed = Org::parse(&opened_file);
    let mut writer = Vec::new();

    // parsed.write_html(&mut writer).unwrap();
    let mut handler = CustomHTMLHandler::default();
    parsed.write_html_custom(&mut writer, &mut handler).unwrap();
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

fn header() -> String {
    String::from(
        "
<html>
<head> <meta charset='utf-8'/> </head>
<body>
	",
    )
}

fn footer() -> String {
    String::from(
        "
	</body></html>
    ",
    )
}

fn template_with_content(content: &str) -> String {
    let mut result = header();
    result.push_str(content);
    result.push_str(&footer());
    result
}

fn tag_page_template() -> Tera {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    let content = template_with_content(
        "
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
    );

    tera.add_raw_template("tag.html", &content)
        .expect("should load raw templat");
    tera
}

fn index_template() -> Tera {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    tera.add_raw_template(
        "index.html",
        &template_with_content(
            "
<ul>
{% for page in pages %}
<li>
  <a href='{{page.title}}.html'>{{page.title}}</a>
</li>
{% endfor %}
</ul>
",
        ),
    )
    .expect("should load raw templat");
    tera
}

fn page_template() -> Tera {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    tera.add_raw_template(
        "page.html",
        &template_with_content(
            "
	{% for tag in tags %}
    <a href='tag-{{tag }}.html'>{{ tag }}</a>
	{% endfor %}
	<div>{{page}}</div>
",
        ),
    )
    .expect("should load raw templat");
    tera
}
