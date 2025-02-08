use gtk::glib;
use gtk4 as gtk;

mod imp;

glib::wrapper! {
    pub struct GtkMdEditor(ObjectSubclass<imp::GtkMdEditor>)
        @extends gtk::TextView, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Scrollable;
}

impl Default for GtkMdEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl GtkMdEditor {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
