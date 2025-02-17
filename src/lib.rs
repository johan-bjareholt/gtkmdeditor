pub mod ffi;
mod parser;
mod widgets;

pub use widgets::GtkMdEditor;

// Ensure the C API is initialized
#[cfg(target_family = "unix")]
#[ctor::ctor]
fn init() {
    // Initialize GObject type system
    if gtk4::init().is_err() {
        eprintln!("Failed to initialize GTK");
        std::process::exit(1);
    }
}
