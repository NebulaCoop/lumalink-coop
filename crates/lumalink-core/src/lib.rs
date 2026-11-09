//! LumaLink's engine: reads a mod in any of the forms people publish, works out what it is, and converts it to a package
//! Nebula loads directly. The command line and the graphical tool are thin layers over this crate.
//!
//! The order of a conversion:
//!
//! 1. [`source::open`] unpacks an archive, or takes a folder as it is.
//! 2. [`layout::Layout::of`] finds the Riivolution patches, disc folders, texture packs and loose files inside.
//! 3. [`riivolution`] reads the patches and their options.
//! 4. [`plan::build`] decides which file goes where, what is for one version only and what players choose, with the reason
//!    for everything it does not use.
//! 5. [`package::write`] checks each file with the converter Nebula uses and writes the
//!    `.luma`: see docs/format.md.

pub mod column_names;
pub mod gecko;
pub mod game_store;
pub mod inspect;
pub mod known_patches;
pub mod object_table;
pub mod layout;
pub mod messages;
pub mod package;
pub mod plan;
pub mod player_constants;
pub mod project;
pub mod region_map;
pub mod riivolution;
pub mod source;
pub mod texture_pack;

/// The package format this version writes; Nebula refuses a newer one.
pub const PACKAGE_FORMAT: u32 = 1;
