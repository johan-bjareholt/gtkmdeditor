use gtk4 as gtk;
use gtk::prelude::*;
use gtk::glib;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.bjareholt.johan.GtkMdEditor")
        .build();

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("GTK Markdown Editor Test")
            .default_width(350)
            .default_height(70)
            .build();

        let mdeditor = gtkmdeditor::GtkMdEditor::new();
        window.set_child(Some(&mdeditor));

        window.present();
    });

    application.run()
}
