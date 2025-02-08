use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.bjareholt.johan.GtkMdEditor")
        .build();

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("GTK Markdown Editor Test")
            .default_width(800)
            .default_height(600)
            .build();

        // Create scrolled window
        let scroll = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .build();

        let mdeditor = gtkmdeditor::GtkMdEditor::new();

        // Add some test markdown content
        let buffer = mdeditor.buffer();
        buffer.set_text(
            "# Markdown Test Document

## Formatting

This is a **bold** text and this is *italic*.
You can also use __bold__ and _italic_ with underscores.

### Links

Here's a [link to Google](https://google.com)
And here's an image: ![cute cat](cat.jpg)

#### Lists and More

##### Small Heading

This editor supports various markdown features!",
        );

        scroll.set_child(Some(&mdeditor));
        window.set_child(Some(&scroll));

        window.present();
    });

    application.run()
}
