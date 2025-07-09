use gtk::glib;
use gtk4 as gtk;

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

    pub fn new_with_image_prefix(md_text: &str, img_prefix: &str) -> Self {
        glib::Object::builder()
            .property("md-text", md_text)
            .property("img-prefix", img_prefix)
            .property("hexpand", &true)
            .property("vexpand", &true)
            .build()
    }
}
