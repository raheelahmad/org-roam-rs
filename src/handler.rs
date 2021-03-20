use super::errors;
use super::reader;
use orgize::export::HtmlHandler;
use std::io::prelude::*;

pub struct CustomHTMLHandler {
    base: orgize::export::DefaultHtmlHandler,
    files: Vec<reader::OrgFile>,
}

impl Default for CustomHTMLHandler {
    fn default() -> Self {
        CustomHTMLHandler {
            base: orgize::export::DefaultHtmlHandler::default(),
            files: Vec::new(),
        }
    }
}
impl CustomHTMLHandler {
    pub fn new<'a>(files: Vec<reader::OrgFile>) -> CustomHTMLHandler {
        CustomHTMLHandler {
            base: orgize::export::DefaultHtmlHandler::default(),
            files,
        }
    }
}

impl HtmlHandler<errors::ExportError> for CustomHTMLHandler {
    fn start<W: Write>(
        &mut self,
        mut w: W,
        element: &orgize::Element,
    ) -> Result<(), errors::ExportError> {
        if let orgize::Element::Link(link) = element {
            if link.path.ends_with("png") {
                let path = &link.path;
                let filename = path
                    .strip_prefix("file:images/")
                    .or_else(|| path.strip_prefix("file:/images/"))
                    .unwrap();

                write!(w, "<img src='/images/{}'/>", filename).unwrap();
            } else if link.path.ends_with("org") {
                // self.files.iter().filter(|f| f.title)
                // if let file = self.files.iter

                let matching_file = self.files.iter().filter(|f| f.path == link.path).last();
                if let Some(a_match) = matching_file {
                    println!("✓ {}", link.path);
                    write!(w, "<a href='{}'/>", a_match.title).unwrap();
                } else {
                    println!("none for {}", link.path);
                    self.base.start(w, element)?;
                }
            } else {
                self.base.start(w, element)?;
            }
        } else {
            self.base.start(w, element)?;
        }
        Ok(())
    }

    fn end<W: Write>(
        &mut self,
        w: W,
        element: &orgize::Element,
    ) -> Result<(), errors::ExportError> {
        self.base.end(w, element)?;
        Ok(())
    }
}
