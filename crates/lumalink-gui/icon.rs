// LumaLink's icon, drawn in code (no picture files): a sky-blue rounded square with two white four-point sparkles.
// Build scripts include this file: lumalink-gui's (the window's icon and the program's), lumalink-setup's and lumalink's.
// tools/icon_files.py draws the same icon as files for the README and web pages (assets/icon): change both together.
//
// - `rgba(size)`: the icon as RGBA pixels, 4 x 4 samples a pixel.
// - `png(size)`: one size as a PNG file.
// - `res(id)`: a Windows resource file (.res) with the icon group `id` in sizes 16 to 256 for the linker: 256 as a PNG,
//   the others as 32-bit bitmaps, the form every Windows version reads.
// - `add_details(res, description, file, version)`: adds the program's details (version, name, publisher) to a .res.

#[allow(dead_code)]
mod icon {
    /// The colour at a point of the unit square (y down): clear outside the rounded square, white on a sparkle.
    fn sample(x: f32, y: f32) -> [f32; 4] {
        // The rounded square: inset 0.04, corner radius 0.22.
        let (inset, radius) = (0.04f32, 0.22f32);
        let half = 0.5 - inset;
        let qx = ((x - 0.5).abs() - (half - radius)).max(0.0);
        let qy = ((y - 0.5).abs() - (half - radius)).max(0.0);
        if (qx * qx + qy * qy).sqrt() > radius {
            return [0.0; 4];
        }
        // Sky blue at the top to deep blue at the bottom (the window's BLUE to BLUE_INK).
        let t = ((y - inset) / (1.0 - 2.0 * inset)).clamp(0.0, 1.0);
        let top = [52.0, 190.0, 240.0];
        let bottom = [20.0, 126.0, 186.0];
        let mut colour = [0.0f32; 4];
        for i in 0..3 {
            colour[i] = top[i] + (bottom[i] - top[i]) * t;
        }
        colour[3] = 255.0;
        // A four-point sparkle: |dx/r|^0.5 + |dy/r|^0.5 <= 1, an astroid with deep curved sides.
        let sparkle = |cx: f32, cy: f32, r: f32| ((x - cx).abs() / r).sqrt() + ((y - cy).abs() / r).sqrt() <= 1.0;
        if sparkle(0.46, 0.55, 0.33) || sparkle(0.74, 0.26, 0.13) {
            colour = [255.0, 255.0, 255.0, 255.0];
        }
        colour
    }

    pub fn rgba(size: u32) -> Vec<u8> {
        const N: u32 = 4;
        let mut out = Vec::with_capacity((size * size * 4) as usize);
        for py in 0..size {
            for px in 0..size {
                let mut sum = [0.0f32; 4];
                for sy in 0..N {
                    for sx in 0..N {
                        let x = (px as f32 + (sx as f32 + 0.5) / N as f32) / size as f32;
                        let y = (py as f32 + (sy as f32 + 0.5) / N as f32) / size as f32;
                        let c = sample(x, y);
                        // Premultiplied while summing, so the edge keeps its colour.
                        for (total, channel) in sum.iter_mut().zip(&c[..3]) {
                            *total += channel * c[3] / 255.0;
                        }
                        sum[3] += c[3];
                    }
                }
                let alpha = sum[3] / (N * N) as f32;
                for total in &sum[..3] {
                    let value = if alpha > 0.0 { total / (N * N) as f32 * 255.0 / alpha } else { 0.0 };
                    out.push(value.round().clamp(0.0, 255.0) as u8);
                }
                out.push(alpha.round().clamp(0.0, 255.0) as u8);
            }
        }
        out
    }

    fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
        out.extend_from_slice(&(data.len() as u32).to_be_bytes());
        let start = out.len();
        out.extend_from_slice(kind);
        out.extend_from_slice(data);
        let crc = crc32fast::hash(&out[start..]);
        out.extend_from_slice(&crc.to_be_bytes());
    }

    pub fn png(size: u32) -> Vec<u8> {
        let pixels = rgba(size);
        let mut raw = Vec::with_capacity(pixels.len() + size as usize);
        for row in pixels.chunks((size * 4) as usize) {
            raw.push(0);
            raw.extend_from_slice(row);
        }
        let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
        let mut header = Vec::new();
        header.extend_from_slice(&size.to_be_bytes());
        header.extend_from_slice(&size.to_be_bytes());
        header.extend_from_slice(&[8, 6, 0, 0, 0]);
        chunk(&mut out, b"IHDR", &header);
        chunk(&mut out, b"IDAT", &miniz_oxide::deflate::compress_to_vec_zlib(&raw, 9));
        chunk(&mut out, b"IEND", &[]);
        out
    }

    pub const SIZES: [u32; 7] = [16, 20, 24, 32, 48, 64, 256];

    fn resource(out: &mut Vec<u8>, kind: u16, id: u16, flags: u16, language: u16, data: &[u8]) {
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&32u32.to_le_bytes()); // header size
        for v in [0xFFFFu16, kind, 0xFFFF, id] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.extend_from_slice(&0u32.to_le_bytes()); // data version
        out.extend_from_slice(&flags.to_le_bytes());
        out.extend_from_slice(&language.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes()); // version
        out.extend_from_slice(&0u32.to_le_bytes()); // characteristics
        out.extend_from_slice(data);
        while out.len() % 4 != 0 {
            out.push(0);
        }
    }

    /// One size as an icon bitmap: BITMAPINFOHEADER (height doubled), BGRA rows bottom-up, then the 1-bit mask.
    fn bitmap(size: u32) -> Vec<u8> {
        let pixels = rgba(size);
        let mask_row = size.div_ceil(32) * 4;
        let mut out = Vec::new();
        for v in [40u32, size, size * 2] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.extend_from_slice(&1u16.to_le_bytes()); // planes
        out.extend_from_slice(&32u16.to_le_bytes()); // bits a pixel
        for v in [0u32, size * size * 4 + mask_row * size, 0, 0, 0, 0] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for y in (0..size).rev() {
            for x in 0..size {
                let p = ((y * size + x) * 4) as usize;
                out.extend_from_slice(&[pixels[p + 2], pixels[p + 1], pixels[p], pixels[p + 3]]);
            }
        }
        for y in (0..size).rev() {
            let mut row = vec![0u8; mask_row as usize];
            for x in 0..size {
                if pixels[((y * size + x) * 4 + 3) as usize] == 0 {
                    row[(x / 8) as usize] |= 0x80 >> (x % 8);
                }
            }
            out.extend_from_slice(&row);
        }
        out
    }

    /// A .res file with the icon group `id` (Windows shows the lowest-numbered group as the program's icon).
    pub fn res(id: u16) -> Vec<u8> {
        const MOVEABLE_DISCARDABLE: u16 = 0x1010;
        const ENGLISH_US: u16 = 0x0409;
        let mut out = Vec::new();
        // The empty entry every .res starts with, all zero: the linker knows a .res by it.
        resource(&mut out, 0, 0, 0, 0, &[]);
        let mut group = Vec::new();
        for v in [0u16, 1, SIZES.len() as u16] {
            group.extend_from_slice(&v.to_le_bytes());
        }
        for (i, size) in SIZES.iter().enumerate() {
            let image = if *size >= 256 { png(*size) } else { bitmap(*size) };
            let image_id = 100 + i as u16;
            resource(&mut out, 3, image_id, MOVEABLE_DISCARDABLE, ENGLISH_US, &image); // RT_ICON
            let side = if *size >= 256 { 0 } else { *size as u8 };
            group.extend_from_slice(&[side, side, 0, 0]);
            group.extend_from_slice(&1u16.to_le_bytes());
            group.extend_from_slice(&32u16.to_le_bytes());
            group.extend_from_slice(&(image.len() as u32).to_le_bytes());
            group.extend_from_slice(&image_id.to_le_bytes());
        }
        resource(&mut out, 14, id, MOVEABLE_DISCARDABLE, ENGLISH_US, &group); // RT_GROUP_ICON
        out
    }

    /// Adds to the .res `out` the program's details (RT_VERSION 1: VS_VERSIONINFO), which Explorer's Properties show,
    /// with the fields Nebula's programs carry (Nebula's cmake/nebula_file_details.rc.in). `version` is the package's
    /// version (major.minor.patch, a pre-release part after it left out of the numbers).
    #[allow(dead_code)]
    pub fn add_details(out: &mut Vec<u8>, description: &str, file: &str, version: &str) {
        let numbers: Vec<u16> = version.split('-').next().unwrap_or("").split('.').map(|p| p.parse().unwrap_or(0)).collect();
        let number = |i: usize| numbers.get(i).copied().unwrap_or(0) as u32;
        let (high, low) = (number(0) << 16 | number(1), number(2) << 16);
        // VS_FIXEDFILEINFO: signature, structure version, file and product versions, flags mask and flags, VOS_NT_WINDOWS32,
        // VFT_APP, no subtype, no date.
        let mut fixed = Vec::new();
        for v in [0xFEEF_04BDu32, 0x0001_0000, high, low, high, low, 0x3F, 0, 0x0004_0004, 1, 0, 0, 0] {
            fixed.extend_from_slice(&v.to_le_bytes());
        }
        let shown = format!("{}.{}.{}", number(0), number(1), number(2));
        let strings: Vec<Vec<u8>> = [
            ("CompanyName", "keepvaibin"),
            ("FileDescription", description),
            ("FileVersion", shown.as_str()),
            ("LegalCopyright", "Copyright 2026 keepvaibin"),
            ("OriginalFilename", file),
            ("ProductName", "LumaLink"),
            ("ProductVersion", shown.as_str()),
        ]
        .iter()
        .map(|(key, value)| {
            let text = wide(value);
            node(key, (text.len() / 2) as u16, 1, &text, &[])
        })
        .collect();
        let table = node("040904B0", 0, 1, &[], &strings); // US English, Unicode
        let string_info = node("StringFileInfo", 0, 1, &[], &[table]);
        let translation = node("Translation", 4, 0, &[0x09, 0x04, 0xB0, 0x04], &[]);
        let var_info = node("VarFileInfo", 0, 1, &[], &[translation]);
        let root = node("VS_VERSION_INFO", fixed.len() as u16, 0, &fixed, &[string_info, var_info]);
        resource(out, 16, 1, 0x0030, 0x0409, &root); // RT_VERSION
    }

    /// `text` as UTF-16 with its ending zero.
    fn wide(text: &str) -> Vec<u8> {
        text.encode_utf16().chain([0]).flat_map(u16::to_le_bytes).collect()
    }

    /// One block of a version resource: its length, its value's length (in characters for text, bytes otherwise), its
    /// kind (1 text, 0 binary), its key, then its value and its children, each on a 4-byte boundary.
    fn node(key: &str, value_length: u16, kind: u16, value: &[u8], children: &[Vec<u8>]) -> Vec<u8> {
        let mut out = vec![0, 0];
        out.extend_from_slice(&value_length.to_le_bytes());
        out.extend_from_slice(&kind.to_le_bytes());
        out.extend_from_slice(&wide(key));
        let align = |out: &mut Vec<u8>| out.resize(out.len().next_multiple_of(4), 0);
        align(&mut out);
        out.extend_from_slice(value);
        for child in children {
            align(&mut out);
            out.extend_from_slice(child);
        }
        let length = out.len() as u16;
        out[..2].copy_from_slice(&length.to_le_bytes());
        out
    }
}
