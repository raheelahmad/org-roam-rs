use errors::ExportError;
use orgize::Org;
use std::{fs::create_dir_all, io::prelude::*, path::Path};

use super::errors;
use super::handler;
use super::orgtag::{OrgFile, OrgTag, Wiki};
use super::templates;

pub fn publish(wiki: Wiki) -> Result<(), ExportError> {
    let base_path = Wiki::base_path();
    if !Path::new(&base_path).exists() {
        create_dir_all(base_path).expect("Should create export directory if it doesn't exist");
    }

    wiki.tags.iter().try_for_each(|tag| publish_tag(tag))?;
    publish_all_pages(&wiki)?;
    wiki.files
        .iter()
        .try_for_each(|file| publish_file(file, &wiki))?;

    copy_images()?;
    copy_assets()?;
    Ok(())
}

fn copy_images() -> Result<(), fs_extra::error::Error> {
    let from = std::path::Path::new("/Users/raheel/orgs/roam/images");
    let to = std::path::Path::new("/Users/raheel/Downloads/org-roam-export");
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    fs_extra::dir::copy(from, to, &options)?;
    Ok(())
}

fn copy_assets() -> Result<(), fs_extra::error::Error> {
    let from = std::path::Path::new("/Users/raheel/orgs/roam/css");
    let to = std::path::Path::new("/Users/raheel/Downloads/org-roam-export");
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    fs_extra::dir::copy(from, to, &options)?;
    Ok(())
}

fn publish_all_pages(wiki: &Wiki) -> Result<(), std::io::Error> {
    let template = templates::all_pages_template();
    let mut context = tera::Context::new();
    context.insert("pages", &wiki.files);
    context.insert("title", "All Pages");
    let render_result = template.render("all_pages.html", &context).unwrap();
    let content_bytes = render_result.into_bytes();
    let path = Wiki::base_path() + "all_pages.html";
    let mut output = std::fs::File::create(path).unwrap();
    output.write_all(&content_bytes)?;

    Ok(())
}

fn publish_tag(tag: &OrgTag) -> Result<(), std::io::Error> {
    let tempalte = templates::tag_page_template();
    let mut context = tera::Context::new();
    context.insert("tag_name", &tag.name);
    context.insert("pages", &tag.files);
    context.insert("title", &tag.name);
    let render_result = tempalte.render("tag.html", &context).unwrap();
    let content_bytes = render_result.into_bytes();
    let mut output = tag.output_file()?;
    output.write_all(&content_bytes)?;

    Ok(())
}

fn publish_file(file: &OrgFile, wiki: &Wiki) -> Result<(), ExportError> {
    let parsed = Org::parse(&file.raw_file);
    let mut writer = Vec::new();

    let files = wiki.files.clone();
    let mut handler = handler::CustomHtmlHandler::new(files);
    parsed.write_html_custom(&mut writer, &mut handler)?;
    let parsed_str = String::from_utf8(writer).unwrap();

    let template = templates::page_template();

    let org_path = file.path.split('/').last().unwrap();
    let referring_files: Vec<&OrgFile> = wiki
        .files
        .iter()
        .filter(|f| f.referenced_file_paths.contains(&org_path.to_string()))
        .collect();

    let mut context = tera::Context::new();
    context.insert("page", &parsed_str);
    context.insert("tags", &file.tags);
    context.insert("title", &file.title);
    context.insert("backlinks", &referring_files);
    let result = template.render("page.html", &context);
    let path = Wiki::base_path() + &file.title + ".html";
    let mut output = std::fs::File::create(path).unwrap();
    let content_bytes = result.unwrap().into_bytes();
    output.write_all(&content_bytes)?;

    Ok(())
}
