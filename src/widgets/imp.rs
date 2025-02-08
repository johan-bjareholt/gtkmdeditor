use crate::parser::{self, Attribute};
use gtk::glib::{self, SignalHandlerId};
use gtk::pango;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{TextBuffer, TextTag};
use gtk4 as gtk;
use std::cell::RefCell;

// Object holding the state
#[derive(Default)]
pub struct GtkMdEditor {
    tags: RefCell<Option<Tags>>,
    buffer_changed_handler: RefCell<Option<SignalHandlerId>>,
}

struct Tags {
    heading1: TextTag,
    heading2: TextTag,
    heading3: TextTag,
    heading4: TextTag,
    heading5: TextTag,
    bold: TextTag,
    italic: TextTag,
    link: TextTag,
}

impl Tags {
    fn new(buffer: &TextBuffer) -> Self {
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

    fn get_tag_for_attr(&self, attr: &Attribute) -> &TextTag {
        match attr {
            Attribute::Heading1 => &self.heading1,
            Attribute::Heading2 => &self.heading2,
            Attribute::Heading3 => &self.heading3,
            Attribute::Heading4 => &self.heading4,
            Attribute::Heading5 => &self.heading5,
            Attribute::Bold => &self.bold,
            Attribute::Italic => &self.italic,
            Attribute::Link | Attribute::Picture => &self.link,
        }
    }
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for GtkMdEditor {
    const NAME: &'static str = "GtkMdEditor";
    type Type = super::GtkMdEditor;
    type ParentType = gtk::TextView;
}

impl GtkMdEditor {
    fn setup_buffer(&self, obj: &super::GtkMdEditor) {
        let buffer = obj.buffer();

        // Initialize tags
        let tags = Tags::new(&buffer);
        *self.tags.borrow_mut() = Some(tags);

        // Connect to buffer changes
        let handler_id = buffer.connect_changed(glib::clone!(@weak obj => move |buffer| {
            let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
            Self::update_highlighting(&obj, buffer, &text);
        }));

        *self.buffer_changed_handler.borrow_mut() = Some(handler_id);
    }

    fn update_highlighting(widget: &super::GtkMdEditor, buffer: &TextBuffer, text: &str) {
        // Remove all existing tags
        buffer.remove_all_tags(&buffer.start_iter(), &buffer.end_iter());

        // Get new attributes from parser
        let attributes = parser::get_attributes(text);

        // Get tags instance
        if let Some(tags) = &*widget.imp().tags.borrow() {
            // Apply tags based on parser results
            for span in attributes {
                let start = buffer.iter_at_offset(span.range.start as i32);
                let end = buffer.iter_at_offset(span.range.end as i32);
                buffer.apply_tag(tags.get_tag_for_attr(&span.attr), &start, &end);
            }
        }
    }
}

// Trait shared by all GObjects
impl ObjectImpl for GtkMdEditor {
    fn constructed(&self) {
        self.parent_constructed();
        self.setup_buffer(&self.obj());
    }
}

// Trait shared by all widgets
impl WidgetImpl for GtkMdEditor {}

// Trait shared by all text views
impl TextViewImpl for GtkMdEditor {}
