use gtk4 as gtk;
use gtk::prelude::*;
use gtk::pango;

use crate::parser::{Block, Attribute};

pub struct Tags {
    heading1: gtk::TextTag,
    heading2: gtk::TextTag,
    heading3: gtk::TextTag,
    heading4: gtk::TextTag,
    heading5: gtk::TextTag,
    bold: gtk::TextTag,
    italic: gtk::TextTag,
    link: gtk::TextTag,
    text: gtk::TextTag,
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

        let text = buffer
            .create_tag(Some("text"), &[])
            .expect("Failed to create text tag");

        Self {
            heading1,
            heading2,
            heading3,
            heading4,
            heading5,
            bold,
            italic,
            link,
            text,
        }
    }

    pub fn get_tag_for_attr(&self, attr: &Attribute) -> &gtk::TextTag {
        match attr {
            Attribute::Heading1(_) => &self.heading1,
            Attribute::Heading2(_) => &self.heading2,
            Attribute::Heading3(_) => &self.heading3,
            Attribute::Heading4(_) => &self.heading4,
            Attribute::Heading5(_) => &self.heading5,
            Attribute::Bold(_) => &self.bold,
            Attribute::Italic(_) => &self.italic,
            Attribute::Link(_) => &self.link,
            Attribute::Picture(_) => &self.link,
            Attribute::Text(_) => &self.text,
        }
    }
}

pub struct GtkMdBlock {
    pub span_start: gtk::TextMark,
    pub span_end: gtk::TextMark,
    pub attr: Attribute,
}

impl GtkMdBlock {
    pub fn new_from_block(block: &Block, text_buffer: &gtk::TextBuffer) -> Self {
        let start_iter = text_buffer.iter_at_offset(block.span.start as i32);
        let end_iter = text_buffer.iter_at_offset(block.span.end as i32);

        let start_mark = text_buffer.create_mark(None, &start_iter, true);
        let end_mark = text_buffer.create_mark(None, &end_iter, false);

        Self {
            span_start: start_mark,
            span_end: end_mark,
            attr: block.attr.clone()
        }
    }

    pub fn apply_block_editor(&self, tags: &Tags) {
        let tag = tags.get_tag_for_attr(&self.attr);
        let buffer = self.span_start.buffer().unwrap();
        let start_iter = buffer.iter_at_mark(&self.span_start);
        let end_iter = buffer.iter_at_mark(&self.span_end);
        buffer.apply_tag(tag, &start_iter, &end_iter);
    }

    pub fn apply_block_viewer(&self, tags: &Tags) {
        let tag = tags.get_tag_for_attr(&self.attr);
        let buffer = self.span_start.buffer().unwrap();
        match &self.attr {
            Attribute::Heading1(text) |
            Attribute::Heading2(text) |
            Attribute::Heading3(text) |
            Attribute::Heading4(text) |
            Attribute::Heading5(text) |
            Attribute::Bold(text) |
            Attribute::Italic(text) |
            Attribute::Link((text, _)) |
            Attribute::Text(text) => {
                buffer.delete(
                    &mut buffer.iter_at_mark(&self.span_start),
                    &mut buffer.iter_at_mark(&self.span_end),
                );
                buffer.insert_with_tags(
                    &mut buffer.iter_at_mark(&self.span_start),
                    text,
                    &[tag]
                );
            },
            Attribute::Picture(_) => {
                buffer.apply_tag(
                    tag,
                    &mut buffer.iter_at_mark(&self.span_start),
                    &mut buffer.iter_at_mark(&self.span_end),
                );
            },
        }
    }
}
