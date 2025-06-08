use gtk4 as gtk;
use gtk::glib::{self, SignalHandlerId};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::TextBuffer;
use std::cell::RefCell;

use crate::parser;
use crate::tags::Tags;

// Object holding the state
#[derive(Default)]
pub struct GtkMdEditor {
    tags: RefCell<Option<Tags>>,
    buffer_changed_handler: RefCell<Option<SignalHandlerId>>,
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
        let handler_id = buffer.connect_changed(glib::clone!(#[weak] obj,  move |buffer| {
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
