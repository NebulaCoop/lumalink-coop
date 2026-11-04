//! New materials a mod brings: their shaders generated from the material, compiled once here for every graphics interface,
//! and stored in the package. The device finishes them for its own chip when they are first used.

use anyhow::{bail, Context, Result};
use nebula_import::cook::shader_source;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};

/// A material's output variants, numbered as Nebula's `NativeShaderVariant`, with their defines.
const VARIANTS: [(u8, &[&str]); 8] = [
    (0, &[]),
    (4, &["NATIVE_DUAL"]),
    (5, &["NATIVE_SAMPLE"]),
    (6, &["NATIVE_SAMPLE", "NATIVE_DUAL"]),
    (7, &["NATIVE_PIXEL_LIT"]),
    (8, &["NATIVE_PIXEL_LIT", "NATIVE_DUAL"]),
    (9, &["NATIVE_PIXEL_LIT", "NATIVE_SAMPLE"]),
    (10, &["NATIVE_PIXEL_LIT", "NATIVE_SAMPLE", "NATIVE_DUAL"]),
];

/// The variant whose vertex stage a per-sample variant uses: only the pixel stage differs.
fn vertex_of(variant: u8) -> u8 {
    match variant {
        5 => 0,
        6 => 4,
        9 => 7,
        10 => 8,
        other => other,
    }
}

/// Whether a material has this variant: per-pixel ones only where it lights, per-sample ones only where it discards.
fn has_variant(source: &str, defines: &[&str]) -> bool {
    (!defines.contains(&"NATIVE_PIXEL_LIT") || source.contains("NATIVE_PIXEL_LIT"))
        && (!defines.contains(&"NATIVE_SAMPLE") || source.contains("discard"))
}

/// The compilers, for a head start: without them Nebula builds a mod's shaders on the player's device.
#[derive(Clone, Debug)]
pub struct Tools {
    pub dxc: PathBuf,
    pub spirv: PathBuf,
}

impl Tools {
    /// From `LUMALINK_DXC` and `LUMALINK_SPIRV`, else a `tools` folder beside this program.
    pub fn find() -> Option<Self> {
        let beside = std::env::current_exe().ok().and_then(|e| e.parent().map(|p| p.join("tools")));
        let found = |variable: &str, name: &str| {
            std::env::var_os(variable).map(PathBuf::from).filter(|p| p.is_file()).or_else(|| {
                let dir = beside.as_ref()?;
                [dir.join(name), dir.join(format!("{name}.exe"))].into_iter().find(|p| p.is_file())
            })
        };
        Some(Self { dxc: found("LUMALINK_DXC", "dxc")?, spirv: found("LUMALINK_SPIRV", "galaxy_native_spirv")? })
    }
}

/// What came of compiling a mod's materials.
#[derive(Default)]
pub struct Compiled {
    pub dxil: Vec<u8>,
    pub spirv: Vec<u8>,
    pub sources: BTreeMap<u32, String>,
    pub shaders: usize,
    /// Materials that could not be made, with the reason: models using them are not drawn.
    pub failed: Vec<(u32, String)>,
}

fn run(mut command: Command, what: &str) -> Result<()> {
    let output = command.output().with_context(|| format!("{what}: the compiler can't be started"))?;
    if !output.status.success() {
        let mut text = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if text.is_empty() {
            text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
        text.truncate(400);
        bail!("{what}: {text}");
    }
    Ok(())
}

/// One material's every stage, for both interfaces: (variant, pixel, bytes).
type Stages = Vec<(u8, bool, Vec<u8>)>;

fn compile_material(id: u32, source: &str, tools: &Tools, work: &Path) -> Result<(Stages, Stages)> {
    let file = work.join(format!("m{id:08x}.hlsl"));
    fs::write(&file, source)?;
    let (mut dxil, mut spirv) = (Stages::new(), Stages::new());
    for (variant, defines) in VARIANTS.iter().filter(|(_, d)| has_variant(source, d)) {
        for (stage, pixel) in [("vs", false), ("ps", true)] {
            // A per-sample variant's vertex stage is another variant's: stored once below.
            if !pixel && vertex_of(*variant) != *variant {
                continue;
            }
            let out = work.join(format!("m{id:08x}_{variant}_{stage}"));
            let (out_dxil, out_spirv) = (out.with_extension("dxil"), out.with_extension("spv"));
            let mut dxc = Command::new(&tools.dxc);
            dxc.args(["-nologo", "-T", &format!("{stage}_6_0"), "-E", &format!("native_{stage}"), "-O3", "-WX", "-Qstrip_debug", "-Qstrip_reflect"]);
            for define in *defines {
                dxc.arg(format!("-D{define}=1"));
            }
            dxc.arg("-I").arg(work).arg("-Fo").arg(&out_dxil).arg(&file);
            run(dxc, "DirectX")?;
            let mut vk = Command::new(&tools.spirv);
            vk.arg(&file).arg(work).arg(stage).arg("shader").arg(&out_spirv).args(*defines);
            run(vk, "Vulkan")?;
            dxil.push((*variant, pixel, fs::read(&out_dxil)?));
            spirv.push((*variant, pixel, fs::read(&out_spirv)?));
            let _ = fs::remove_file(out_dxil);
            let _ = fs::remove_file(out_spirv);
        }
    }
    let _ = fs::remove_file(&file);
    Ok((dxil, spirv))
}

const HEADER_BYTES: usize = 56;

/// The file format of Nebula's `mod_shaders.h`.
fn blob(materials: &BTreeMap<u32, Stages>, interface: &str) -> Vec<u8> {
    // (material, variant, pixel, the variant whose stage holds the bytes)
    let mut rows = Vec::new();
    for (&id, stages) in materials {
        for (variant, pixel, _) in stages {
            rows.push((id, *variant, *pixel));
            if *pixel && !stages.iter().any(|(v, p, _)| v == variant && !*p) {
                rows.push((id, *variant, false));
            }
        }
    }
    let table = HEADER_BYTES + rows.len() * 24;
    let mut out = vec![0u8; table];
    out[..8].copy_from_slice(b"NBSHADER");
    out[8..12].copy_from_slice(&1u32.to_le_bytes());
    out[12..16].copy_from_slice(&(rows.len() as u32).to_le_bytes());
    out[16..56].copy_from_slice(interface.as_bytes());
    let mut places: BTreeMap<(u32, u8, bool), (u64, u64)> = BTreeMap::new();
    for (&id, stages) in materials {
        for (variant, pixel, bytes) in stages {
            while out.len() % 8 != 0 {
                out.push(0);
            }
            places.insert((id, *variant, *pixel), (out.len() as u64, bytes.len() as u64));
            out.extend_from_slice(bytes);
        }
    }
    for (i, &(id, variant, pixel)) in rows.iter().enumerate() {
        let holder = if pixel { variant } else { vertex_of(variant) };
        let (offset, size) = places[&(id, holder, pixel)];
        let at = HEADER_BYTES + i * 24;
        out[at..at + 4].copy_from_slice(&id.to_le_bytes());
        out[at + 4] = variant;
        out[at + 5] = pixel as u8;
        out[at + 8..at + 16].copy_from_slice(&offset.to_le_bytes());
        out[at + 16..at + 24].copy_from_slice(&size.to_le_bytes());
    }
    out
}

/// Generates and compiles every material, in parallel, under `work`.
pub fn compile(materials: &BTreeMap<u32, Vec<u8>>, tools: &Tools, work: &Path) -> Result<Compiled> {
    fs::create_dir_all(work)?;
    for (name, text) in shader_source::HEADERS {
        fs::write(work.join(name), text)?;
    }
    let mut out = Compiled::default();
    let mut jobs = Vec::new();
    for (&id, record) in materials {
        match shader_source::material_source(record) {
            Ok(source) => jobs.push((id, source)),
            Err(error) => out.failed.push((id, format!("{error:#}"))),
        }
    }
    let queue = Arc::new(Mutex::new(jobs.clone()));
    let done: Arc<Mutex<Vec<(u32, Result<(Stages, Stages)>)>>> = Arc::new(Mutex::new(Vec::new()));
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get()).clamp(1, 8);
    std::thread::scope(|scope| {
        for _ in 0..threads {
            let (queue, done) = (queue.clone(), done.clone());
            scope.spawn(move || loop {
                let Some((id, source)) = queue.lock().unwrap_or_else(|e| e.into_inner()).pop() else {
                    break;
                };
                let result = compile_material(id, &source, tools, work);
                done.lock().unwrap_or_else(|e| e.into_inner()).push((id, result));
            });
        }
    });
    let (mut dxil, mut spirv) = (BTreeMap::new(), BTreeMap::new());
    let finished = std::mem::take(&mut *done.lock().unwrap_or_else(|e| e.into_inner()));
    for (id, result) in finished {
        match result {
            Ok((d, s)) => {
                out.shaders += d.len();
                dxil.insert(id, d);
                spirv.insert(id, s);
            }
            Err(error) => out.failed.push((id, format!("{error:#}"))),
        }
    }
    out.failed.sort();
    let interface = shader_source::interface_id();
    out.dxil = blob(&dxil, &interface);
    out.spirv = blob(&spirv, &interface);
    let succeeded: Vec<u32> = dxil.keys().copied().collect();
    out.sources = jobs.into_iter().filter(|(id, _)| succeeded.contains(id)).collect();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_follow_what_the_material_does() {
        assert!(has_variant("plain", &[]));
        assert!(!has_variant("plain", &["NATIVE_PIXEL_LIT"]));
        assert!(!has_variant("plain", &["NATIVE_SAMPLE"]));
        assert!(has_variant("#if NATIVE_PIXEL_LIT discard", &["NATIVE_PIXEL_LIT", "NATIVE_SAMPLE"]));
        assert_eq!(vertex_of(9), 7);
        assert_eq!(vertex_of(0), 0);
    }

    #[test]
    fn a_blob_lists_every_stage_and_shares_vertex_stages() {
        let mut materials = BTreeMap::new();
        materials.insert(
            0x4000_0001u32,
            vec![(0u8, false, vec![1u8, 2, 3]), (0, true, vec![4]), (5, true, vec![5, 6])],
        );
        let interface = "0123456789012345678901234567890123456789";
        let bytes = blob(&materials, interface);
        assert_eq!(&bytes[16..56], interface.as_bytes());
        assert_eq!(&bytes[..8], b"NBSHADER");
        assert_eq!(u32::from_le_bytes(bytes[12..16].try_into().unwrap()), 4);
        let entry = |i: usize| {
            let at = HEADER_BYTES + i * 24;
            let id = u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
            let offset = u64::from_le_bytes(bytes[at + 8..at + 16].try_into().unwrap()) as usize;
            let size = u64::from_le_bytes(bytes[at + 16..at + 24].try_into().unwrap()) as usize;
            (id, bytes[at + 4], bytes[at + 5], bytes[offset..offset + size].to_vec())
        };
        assert_eq!(entry(0), (0x4000_0001, 0, 0, vec![1, 2, 3]));
        assert_eq!(entry(1), (0x4000_0001, 0, 1, vec![4]));
        assert_eq!(entry(2), (0x4000_0001, 5, 1, vec![5, 6]));
        // Variant 5's vertex stage is variant 0's.
        assert_eq!(entry(3), (0x4000_0001, 5, 0, vec![1, 2, 3]));
    }
}
