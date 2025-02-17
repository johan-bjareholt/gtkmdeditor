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

    // Copy header file to include directory
    let header_src = Path::new(&manifest_dir).join("src/ffi/gtkmdeditor.h");
    let header_dest = Path::new(&build_dir).join("include/gtkmdeditor/gtkmdeditor.h");
    fs::create_dir_all(header_dest.parent().unwrap()).unwrap();
    fs::copy(header_src, header_dest).unwrap();

    println!("cargo:rerun-if-changed=src/ffi/gtkmdeditor.h");
}
