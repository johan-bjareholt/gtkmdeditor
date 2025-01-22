use gtk4 as gtk;
use gtk::glib;
use glib::prelude::*;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default)]
pub struct GtkMdEditor;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for GtkMdEditor {
    const NAME: &'static str = "GtkMdEditor";
    type Type = super::GtkMdEditor;
    type ParentType = gtk::TextView;
}

impl GtkMdEditor {}

// Trait shared by all GObjects
impl ObjectImpl for GtkMdEditor {}

// Trait shared by all widgets
impl WidgetImpl for GtkMdEditor {}

// Trait shared by all boxes
impl TextViewImpl for GtkMdEditor {}
