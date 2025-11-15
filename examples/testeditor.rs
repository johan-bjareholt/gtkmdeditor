use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;
use std::fs;
use std::env;

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

        // Get file path from args or use default
        let args: Vec<String> = env::args().collect();
        let file_path = if args.len() > 1 {
            &args[1]
        } else {
            "examples/test.md"
        };

        // Read markdown from file
        let markdown_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", file_path, e);
                format!("# Error\n\nCould not read file: {}\n\nError: {}", file_path, e)
            }
        };

        // Set the content in the editor
        let buffer = mdeditor.buffer();
        buffer.set_text(&markdown_content);

        scroll.set_child(Some(&mdeditor));
        window.set_child(Some(&scroll));

        window.present();
    });

    application.run_with_args::<&String>(&[])
}
