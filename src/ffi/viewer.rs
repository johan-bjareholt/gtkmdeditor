use std::ffi::{c_void, c_char, CStr};

use gtk4::glib;
use gtk4::glib::translate::*;
use gtk4::prelude::*;

use crate::GtkMdViewer;

#[unsafe(no_mangle)]
pub extern "C" fn gtk_md_viewer_get_type() -> glib::Type {
    GtkMdViewer::static_type()
}

#[unsafe(no_mangle)]
pub extern "C" fn gtk_md_viewer_new(md_text_ptr: *const c_char) -> *mut c_void {
    let md_text = unsafe { CStr::from_ptr(md_text_ptr) };
    let viewer = GtkMdViewer::new(md_text.to_str().unwrap());
    // Cast to Widget first, then convert to pointer
    let widget = viewer.upcast_ref::<gtk4::Widget>();
    let ptr: *mut gtk4::ffi::GtkWidget = widget.to_glib_full();
    ptr as *mut c_void
}

#[unsafe(no_mangle)]
pub extern "C" fn gtk_md_viewer_new_with_img_prefix(md_text_ptr: *const c_char, img_prefix_ptr: *const c_char) -> *mut c_void {
    let md_text = unsafe { CStr::from_ptr(md_text_ptr) };
    let img_prefix = unsafe { CStr::from_ptr(img_prefix_ptr) };
    let viewer = GtkMdViewer::new_with_image_prefix(md_text.to_str().unwrap(), img_prefix.to_str().unwrap());
    // Cast to Widget first, then convert to pointer
    let widget = viewer.upcast_ref::<gtk4::Widget>();
    let ptr: *mut gtk4::ffi::GtkWidget = widget.to_glib_full();
    ptr as *mut c_void
}
