use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;
use std::fs;
use std::env;

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

        // Get file path from args or use default
        let args: Vec<String> = env::args().collect();
        let file_path = if args.len() > 1 {
            &args[1]
        } else {
            "test.md"
        };

        // Read markdown from file
        let (markdown_content, img_prefix) = match fs::read_to_string(file_path) {
            Ok(content) => {
                // Use the directory of the markdown file as image prefix
                let img_prefix = std::path::Path::new(file_path)
                    .parent()
                    .and_then(|p| p.to_str())
                    .unwrap_or("./");
                (content, img_prefix.to_string())
            }
            Err(e) => {
                eprintln!("Error reading file '{}': {}", file_path, e);
                (format!("# Error\n\nCould not read file: {}\n\nError: {}", file_path, e), "./".to_string())
            }
        };

        let mdviewer = gtkmdeditor::GtkMdViewer::new_with_image_prefix(&markdown_content, &img_prefix);

        scroll.set_child(Some(&mdviewer));
        window.set_child(Some(&scroll));

        window.present();
    });

    application.run()
}
