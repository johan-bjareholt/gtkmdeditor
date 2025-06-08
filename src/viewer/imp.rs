use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;

use crate::parser;
use crate::tags::Tags;

// Object holding the state
#[derive(Default)]
pub struct GtkMdViewer {}

impl GtkMdViewer {
    fn setup_buffer(&self, buffer: &gtk::TextBuffer) {
        // Initialize tags
        let tags = Tags::new(&buffer);

        // Apply tags
        let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
        println!("buffer text is: {}", text);

        // Get attributes from parser
        let attributes = parser::get_attributes(&text);

        // Apply tags based on parser results
        for span in attributes {
            let start = buffer.iter_at_offset(span.range.start as i32);
            let end = buffer.iter_at_offset(span.range.end as i32);
            buffer.apply_tag(tags.get_tag_for_attr(&span.attr), &start, &end);
        }
    }
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for GtkMdViewer {
    const NAME: &'static str = "GtkMdViewer";
    type Type = super::GtkMdViewer;
    type ParentType = gtk::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for GtkMdViewer {
    fn constructed(&self) {
        self.parent_constructed();
    }

    fn properties() -> &'static [glib::ParamSpec] {
        use std::sync::OnceLock;

        static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
        PROPERTIES.get_or_init(|| {
            vec![
                glib::ParamSpecString::builder("md-text")
                    .nick("Markdown text")
                    .blurb("Markdown to render")
                    .default_value(Some(""))
                    .flags(glib::ParamFlags::WRITABLE | glib::ParamFlags::CONSTRUCT_ONLY)
                    .build(),
            ]
        })
    }

    fn set_property(
        &self,
        _id: usize,
        value: &glib::Value,
        pspec: &glib::ParamSpec,
    ) {
        match pspec.name() {
            "md-text" => {
                let md_text = value.get::<String>().expect("Type checked by GObject");

                let buffer = gtk::TextBuffer::new(None);
                buffer.set_text(&md_text);
                self.setup_buffer(&buffer);

                let obj = self.obj();
                obj.setup(&buffer);
            }
            _ => unimplemented!(),
        }
    }

}

// Trait shared by all widgets
impl WidgetImpl for GtkMdViewer {}

// Trait shared by all boxes
impl BoxImpl for GtkMdViewer {}
