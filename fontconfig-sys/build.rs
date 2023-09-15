fn main() {
    let dlopen = std::env::var_os("RUST_FONTCONFIG_DLOPEN").is_some();
    println!("cargo:rerun-if-env-changed=RUST_FONTCONFIG_DLOPEN");
    if !cfg!(windows) || dlopen {
        if dlopen {
            println!("cargo:rustc-cfg=feature=\"dlopen\"");
        }
        if !(dlopen || cfg!(feature = "dlopen")) {
            pkg_config::find_library("fontconfig").unwrap();
        }
    } else if cfg!(windows) {
        println!("cargo:rustc-link-lib=fontconfig");
        println!("cargo:rustc-link-lib=freetype");
    }
}
