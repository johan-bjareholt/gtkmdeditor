use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let build_dir = Path::new(&out_dir).join("../../../");

    // Generate pkg-config file
    let pc_file = format!(
        "prefix=/usr/local\n\
         exec_prefix=${{prefix}}\n\
         libdir=${{exec_prefix}}/lib\n\
         includedir=${{prefix}}/include\n\
         \n\
         Name: gtkmdeditor\n\
         Description: GTK Markdown Editor Widget\n\
         Version: {}\n\
         Requires: gtk4\n\
         Libs: -L${{libdir}} -lgtkmdeditor\n\
         Cflags: -I${{includedir}}/gtkmdeditor\n",
        env::var("CARGO_PKG_VERSION").unwrap()
    );
    let pc_path = Path::new(&build_dir).join("gtkmdeditor.pc");
    fs::write(&pc_path, pc_file).unwrap();

    // Copy headers to include directory
    let headers_src = Path::new(&manifest_dir).join("src/ffi/");
    let headers_dest = Path::new(&build_dir).join("include/gtkmdeditor/");
    fs::create_dir_all(&headers_dest).unwrap();
    fs::copy(
        Path::new(&headers_src).join("gtkmdeditor.h"),
        Path::new(&headers_dest).join("gtkmdeditor.h")
    ).unwrap();
    fs::copy(
        Path::new(&headers_src).join("gtkmdviewer.h"),
        Path::new(&headers_dest).join("gtkmdviewer.h")
    ).unwrap();

    println!("cargo:rerun-if-changed=src/ffi/gtkmdeditor.h");
    println!("cargo:rerun-if-changed=src/ffi/gtkmdviewer.h");
}
