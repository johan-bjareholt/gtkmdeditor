use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;

static EXAMPLE_TEXT: &str = "# Markdown Test Document

## Formatting

This is a **bold** text and this is *italic*.
You can also use __bold__ and _italic_ with underscores.

### Links

Here's a [link to Google](https://google.com)
And here's an image: ![cute cat](cat.jpg)

#### Lists and More

##### Small Heading

This viewer supports various markdown features!";

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.bjareholt.johan.GtkMdViewer")
        .build();

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("GTK Markdown Viewer Test")
            .default_width(800)
            .default_height(600)
            .build();

        // Create scrolled window
        let scroll = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .build();

        let mdviewer = gtkmdeditor::GtkMdViewer::new_with_image_prefix(EXAMPLE_TEXT, "./");

        scroll.set_child(Some(&mdviewer));
        window.set_child(Some(&scroll));

        window.present();
    });

    application.run()
}
