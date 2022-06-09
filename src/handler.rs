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

impl HtmlHandler<errors::Error> for CustomHtmlHandler {
    fn start<W: Write>(
        &mut self,
        mut w: W,
        element: &orgize::Element,
    ) -> Result<(), errors::Error> {
        if let orgize::Element::Link(link) = element {
            if link.path.ends_with("png") && !link.path.starts_with("http") {
                let path = &link.path;
                let filename = path
                    .strip_prefix("file:images/")
                    .or_else(|| path.strip_prefix("file:/images/"))
                    .unwrap();

                write!(w, "<img src='images/{}'/>", filename).unwrap();
            } else if link.path.starts_with("id:") {
                // Need to switch out an org fle link with the published file URL
                let link_id = link.path.strip_prefix("id:").unwrap();
                let matching_file = self.files.iter().filter(|f| f.id == link_id).last();
                if let Some(a_match) = matching_file {
                    write!(w, "<a href='{}.html'>{}</a>", a_match.title, a_match.title).unwrap();
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

    fn end<W: Write>(&mut self, w: W, element: &orgize::Element) -> Result<(), errors::Error> {
        self.base.end(w, element)?;
        Ok(())
    }
}
