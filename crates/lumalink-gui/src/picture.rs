//! A picture of a window, for checking its layout (the debug builds' LUMALINK_SHOT, in the LumaLink window and Setup).

use eframe::egui;
use std::path::Path;

/// A 32-bit top-down BMP.
pub fn write_bmp(file: &Path, image: &egui::ColorImage) -> std::io::Result<()> {
    let [w, h] = image.size;
    let mut out = Vec::with_capacity(54 + 4 * w * h);
    let data = (4 * w * h) as u32;
    out.extend_from_slice(b"BM");
    out.extend_from_slice(&(54 + data).to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&54u32.to_le_bytes());
    out.extend_from_slice(&40u32.to_le_bytes());
    out.extend_from_slice(&(w as i32).to_le_bytes());
    out.extend_from_slice(&(-(h as i32)).to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&32u16.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&data.to_le_bytes());
    out.extend_from_slice(&[0u8; 16]);
    for p in &image.pixels {
        out.extend_from_slice(&[p.b(), p.g(), p.r(), 255]);
    }
    std::fs::write(file, out)
}
