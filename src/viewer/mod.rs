use gtk::glib;
use gtk4 as gtk;
use gtk::prelude::*;
use std::path::Path;

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



    fn add_images(&self, images: &[(String, String)], img_prefix: &str) {
        let flowbox = gtk::FlowBox::new();
        flowbox.set_selection_mode(gtk::SelectionMode::None);
        
        for (_alt_text, image_path) in images {
            let full_path = if img_prefix.is_empty() {
                image_path.clone()
            } else {
                Path::new(img_prefix).join(image_path).to_string_lossy().to_string()
            };
            
            let picture = gtk::Picture::for_filename(&full_path);
            picture.set_keep_aspect_ratio(true);
            picture.set_can_shrink(true);
            
            flowbox.insert(&picture, -1);
        }
        
        self.append(&flowbox);
    }
}
