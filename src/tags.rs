use gtk4 as gtk;
use gtk::prelude::*;
use gtk::pango;

use crate::parser::Attribute;

pub struct Tags {
    heading1: gtk::TextTag,
    heading2: gtk::TextTag,
    heading3: gtk::TextTag,
    heading4: gtk::TextTag,
    heading5: gtk::TextTag,
    bold: gtk::TextTag,
    italic: gtk::TextTag,
    link: gtk::TextTag,
}

impl Tags {
    pub fn new(buffer: &gtk::TextBuffer) -> Self {
        let heading1 = buffer
            .create_tag(Some("heading1"), &[("scale", &2.0f64), ("weight", &800i32)])
            .expect("Failed to create heading1 tag");

        let heading2 = buffer
            .create_tag(Some("heading2"), &[("scale", &1.8f64), ("weight", &700i32)])
            .expect("Failed to create heading2 tag");

        let heading3 = buffer
            .create_tag(Some("heading3"), &[("scale", &1.6f64), ("weight", &600i32)])
            .expect("Failed to create heading3 tag");

        let heading4 = buffer
            .create_tag(Some("heading4"), &[("scale", &1.4f64), ("weight", &500i32)])
            .expect("Failed to create heading4 tag");

        let heading5 = buffer
            .create_tag(Some("heading5"), &[("scale", &1.2f64), ("weight", &400i32)])
            .expect("Failed to create heading5 tag");

        let bold = buffer
            .create_tag(Some("bold"), &[("weight", &700i32)])
            .expect("Failed to create bold tag");

        let italic = buffer
            .create_tag(Some("italic"), &[("style", &pango::Style::Italic)])
            .expect("Failed to create italic tag");

        let link = buffer
            .create_tag(
                Some("link"),
                &[
                    ("underline", &pango::Underline::Single),
                    ("foreground", &"#0366d6"),
                ],
            )
            .expect("Failed to create link tag");

        Self {
            heading1,
            heading2,
            heading3,
            heading4,
            heading5,
            bold,
            italic,
            link,
        }
    }

    pub fn get_tag_for_attr(&self, attr: &Attribute) -> &gtk::TextTag {
        match attr {
            Attribute::Heading1(_) => &self.heading1,
            Attribute::Heading2(_) => &self.heading2,
            Attribute::Heading3(_) => &self.heading3,
            Attribute::Heading4(_) => &self.heading4,
            Attribute::Heading5(_) => &self.heading5,
            Attribute::Bold => &self.bold,
            Attribute::Italic => &self.italic,
            Attribute::Link | Attribute::Picture => &self.link,
        }
    }
}
