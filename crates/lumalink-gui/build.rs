//! The icon (icon.rs draws it): `OUT_DIR/icon.rgba`, 256 x 256 RGBA for the window; `OUT_DIR/icon.png`, the same as a
//! file, which the Linux release archive carries for the menu entry; and on Windows (MSVC) a resource file linked into
//! lumalink-gui.exe: the icon Explorer, the taskbar and shortcuts show, and the program's details (version, name, publisher).

include!("icon.rs");

fn main() {
    println!("cargo:rerun-if-changed=icon.rs");
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    std::fs::write(out.join("icon.rgba"), icon::rgba(256)).unwrap();
    std::fs::write(out.join("icon.png"), icon::png(256)).unwrap();
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        let res = out.join("icon.res");
        let mut bytes = icon::res(1);
        icon::add_details(&mut bytes, "LumaLink", "lumalink-gui.exe", env!("CARGO_PKG_VERSION"));
        std::fs::write(&res, bytes).unwrap();
        println!("cargo:rustc-link-arg-bins={}", res.display());
    }
}
