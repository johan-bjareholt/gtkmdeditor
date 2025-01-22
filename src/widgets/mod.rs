use gtk4 as gtk;
use gtk::prelude::*;
use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct GtkMdEditor(ObjectSubclass<imp::GtkMdEditor>)
        @extends gtk::TextView, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GtkMdEditor {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
