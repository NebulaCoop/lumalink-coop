//! On Windows (MSVC) a resource file linked into lumalink.exe: LumaLink's icon (lumalink-gui's icon.rs draws it) and the
//! program's details (version, name, publisher), which Explorer's Properties show.

include!("../lumalink-gui/icon.rs");

fn main() {
    println!("cargo:rerun-if-changed=../lumalink-gui/icon.rs");
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        let res = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("details.res");
        let mut bytes = icon::res(1);
        icon::add_details(&mut bytes, "LumaLink command line", "lumalink.exe", env!("CARGO_PKG_VERSION"));
        std::fs::write(&res, bytes).unwrap();
        println!("cargo:rustc-link-arg-bins={}", res.display());
    }
}
