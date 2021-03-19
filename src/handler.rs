use super::errors;
use orgize::export::HtmlHandler;
use std::io::prelude::*;

#[derive(Default)]
pub struct CustomHTMLHandler(orgize::export::DefaultHtmlHandler);

impl HtmlHandler<errors::ExportError> for CustomHTMLHandler {
    fn start<W: Write>(
        &mut self,
        mut w: W,
        element: &orgize::Element,
    ) -> Result<(), errors::ExportError> {
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

    fn end<W: Write>(
        &mut self,
        w: W,
        element: &orgize::Element,
    ) -> Result<(), errors::ExportError> {
        self.0.end(w, element)?;
        Ok(())
    }
}
