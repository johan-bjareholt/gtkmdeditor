use gtk::glib;
use gtk4 as gtk;
use gtk::prelude::*;

mod imp;

glib::wrapper! {
    pub struct GtkMdViewer(ObjectSubclass<imp::GtkMdViewer>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl GtkMdViewer {
    pub fn new(md_text: &str) -> Self {
        glib::Object::builder()
            .property("md-text", md_text)
            .property("hexpand", &true)
            .property("vexpand", &true)
            .build()
    }

    fn setup(&self, buffer: &gtk::TextBuffer) {
        let textview = gtk::TextView::with_buffer(buffer);
        textview.set_property("hexpand", true);
        textview.set_property("vexpand", true);
        textview.set_property("editable", false);
        let widget = textview.upcast_ref::<gtk::Widget>();
        self.append(widget);
    }
}
