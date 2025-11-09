use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;

use crate::parser::{self, Attribute, Block};
use crate::tags::{Tags, GtkMdBlock};

#[derive(Debug)]
enum ContentBlock {
    TextBlock {
        start: usize,
        end: usize,
        blocks: Vec<Block>,
    },
    ImageBlock {
        images: Vec<(String, String)>,
    },
}

// Object holding the state
#[derive(Default)]
pub struct GtkMdViewer {
    img_prefix: std::cell::RefCell<String>,
    md_text: std::cell::RefCell<String>,
}

impl GtkMdViewer {
    fn render_content(&self, md_text: &str, img_prefix: &str) {
        let blocks = parser::get_blocks(md_text);

        // Group blocks into content blocks
        let content_blocks = self.group_into_content_blocks(blocks);

        // Render each content block
        for content_block in content_blocks {
            match content_block {
                ContentBlock::TextBlock { start, end, blocks } => {
                    let text_widget = self.render_text_block(md_text, start, end, &blocks);
                    text_widget.set_valign(gtk::Align::Start);
                    let obj = self.obj();
                    obj.append(&text_widget);
                }
                ContentBlock::ImageBlock { images } => {
                    let image_block_widget = self.render_image_block(&images, img_prefix);
                    let obj = self.obj();
                    image_block_widget.set_halign(gtk::Align::Start);
                    image_block_widget.set_valign(gtk::Align::Start);
                    obj.append(&image_block_widget);
                }
            }
        }
    }

    fn render_image_block(&self, images: &[(String, String)], img_prefix: &str) -> gtk::Widget {
        let flowbox = gtk::FlowBox::new();
        flowbox.set_selection_mode(gtk::SelectionMode::None);

        for (_alt_text, image_path) in images {
            let full_path = if img_prefix.is_empty() {
                image_path.clone()
            } else {
                std::path::Path::new(img_prefix).join(image_path).to_string_lossy().to_string()
            };

            let picture = gtk::Picture::for_filename(&full_path);
            picture.set_content_fit(gtk::ContentFit::Contain);
            picture.set_halign(gtk::Align::Start);
            picture.set_valign(gtk::Align::Start);
            picture.set_size_request(200, 150);

            flowbox.insert(&picture, -1);
        }

        flowbox.upcast()
    }

    fn group_into_content_blocks(&self, blocks: Vec<Block>) -> Vec<ContentBlock> {
        let mut content_blocks = Vec::new();
        let mut current_text_blocks: Vec<Block> = Vec::new();
        let mut current_images: Vec<(String, String)> = Vec::new();

        for block in blocks {
            match &block.attr {
                Attribute::Picture((alt, path)) => {
                    // If we have accumulated text blocks, create a TextBlock
                    if !current_text_blocks.is_empty() {
                        let start = current_text_blocks.iter().map(|b| b.span.start).min().unwrap();
                        let end = current_text_blocks.iter().map(|b| b.span.end).max().unwrap();
                        content_blocks.push(ContentBlock::TextBlock {
                            start,
                            end,
                            blocks: std::mem::take(&mut current_text_blocks),
                        });
                    }

                    // Add image to current image list
                    current_images.push((alt.clone(), path.clone()));
                }
                _ => {
                    // If we have accumulated images, create an ImageBlock
                    if !current_images.is_empty() {
                        content_blocks.push(ContentBlock::ImageBlock {
                            images: std::mem::take(&mut current_images),
                        });
                    }

                    // Add to current text blocks
                    current_text_blocks.push(block);
                }
            }
        }

        // Handle remaining blocks
        if !current_text_blocks.is_empty() {
            let start = current_text_blocks.iter().map(|b| b.span.start).min().unwrap();
            let end = current_text_blocks.iter().map(|b| b.span.end).max().unwrap();
            content_blocks.push(ContentBlock::TextBlock {
                start,
                end,
                blocks: current_text_blocks,
            });
        }

        if !current_images.is_empty() {
            content_blocks.push(ContentBlock::ImageBlock {
                images: current_images,
            });
        }

        content_blocks
    }

    fn render_text_block(&self, md_text: &str, start: usize, end: usize, blocks: &[Block]) -> gtk::Widget {
        // Extract the text for this block
        let text_content = &md_text[start..end];

        // Create buffer and set text
        let buffer = gtk::TextBuffer::new(None);
        buffer.set_text(text_content);

        // Create tags
        let tags = Tags::new(&buffer);

        // Convert all blocks to GtkMdBlocks first
        let gtk_blocks: Vec<GtkMdBlock> = blocks.iter().map(|block| {
            let adjusted_block = Block {
                span: (block.span.start - start)..(block.span.end - start),
                attr: block.attr.clone(),
            };
            GtkMdBlock::new_from_block(&adjusted_block, &buffer)
        }).collect();

        // Then apply all the blocks (may need to be in reverse order like the original code)
        for gtk_block in gtk_blocks.iter().rev() {
            gtk_block.apply_block_viewer(&tags);
        }

        // Create and return the text view
        let textview = gtk::TextView::with_buffer(&buffer);
        textview.set_property("editable", false);
        textview.set_property("wrap-mode", gtk::WrapMode::Word);

        textview.upcast()
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

        // Set vertical orientation for the box
        let obj = self.obj();
        obj.set_orientation(gtk::Orientation::Vertical);

        // Render content after construction when both properties are available
        let md_text = self.md_text.borrow();
        let img_prefix = self.img_prefix.borrow();

        // This sets the same background colour as textview.
        // See here why this only works with libadwaita:
        // https://discourse.gnome.org/t/replacement-for-gtk-style-context-get-color/23026
        #[cfg(feature = "adw")]
        {
            let widget = obj.clone().upcast::<gtk::Widget>();
            widget.add_css_class("view");
        }

        self.render_content(&md_text, &img_prefix);
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
                glib::ParamSpecString::builder("img-prefix")
                    .nick("Image prefix")
                    .blurb("Prefix path for images")
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
                *self.md_text.borrow_mut() = md_text;
            }
            "img-prefix" => {
                let img_prefix = value.get::<String>().expect("Type checked by GObject");
                *self.img_prefix.borrow_mut() = img_prefix;
            }
            _ => {}
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "img-prefix" => self.img_prefix.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

}

// Trait shared by all widgets
impl WidgetImpl for GtkMdViewer {}

// Trait shared by all boxes
impl BoxImpl for GtkMdViewer {}
