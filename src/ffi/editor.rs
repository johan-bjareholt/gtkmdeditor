use std::ffi::c_void;

use gtk4::glib;
use gtk4::glib::translate::*;
use gtk4::prelude::*;

use crate::GtkMdEditor;

#[unsafe(no_mangle)]
pub extern "C" fn gtk_md_editor_get_type() -> glib::Type {
    GtkMdEditor::static_type()
}

#[unsafe(no_mangle)]
pub extern "C" fn gtk_md_editor_new() -> *mut c_void {
    let editor = GtkMdEditor::new();
    // Cast to Widget first, then convert to pointer
    let widget = editor.upcast_ref::<gtk4::Widget>();
    let ptr: *mut gtk4::ffi::GtkWidget = widget.to_glib_full();
    ptr as *mut c_void
}
