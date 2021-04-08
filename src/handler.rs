use super::errors;
use super::orgtag::OrgFile;
use orgize::export::HtmlHandler;
use std::io::prelude::*;

pub struct CustomHtmlHandler {
    base: orgize::export::DefaultHtmlHandler,
    files: Vec<OrgFile>,
}

impl Default for CustomHtmlHandler {
    fn default() -> Self {
        CustomHtmlHandler {
            base: orgize::export::DefaultHtmlHandler::default(),
            files: Vec::new(),
        }
    }
}
impl CustomHtmlHandler {
    pub fn new(files: Vec<OrgFile>) -> CustomHtmlHandler {
        CustomHtmlHandler {
            base: orgize::export::DefaultHtmlHandler::default(),
            files,
        }
    }
}

impl HtmlHandler<errors::ExportError> for CustomHtmlHandler {
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
            } else if link.path.ends_with("org") && !link.path.starts_with("http") {
                // Need to switch out an org fle link with the published file URL
                let link_path = link.path.strip_prefix("file:").unwrap();
                let matching_file = self
                    .files
                    .iter()
                    .filter(|f| {
                        let file_path_comp = f.path.split('/').last().unwrap();
                        file_path_comp == link_path
                    })
                    .last();
                if let Some(a_match) = matching_file {
                    write!(w, "<a href='{}'>{}</a>", a_match.title, a_match.title).unwrap();
                } else {
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
