//! The command line: `lumalink inspect`, `convert` and `batch`.

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use lumalink_core::game_store::GameStore;
use lumalink_core::layout::Layout;
use lumalink_core::plan::{self, Level, Plan};
use lumalink_core::{package, project, source};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "lumalink", version, about = "Converts Super Mario Galaxy mods and texture packs so Nebula runs them natively.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Args)]
struct Game {
    /// Your imported game (Nebula's game.pak). Without it, mods that name files only by their name can't be placed.
    #[arg(long, value_name = "FILE")]
    game: Option<PathBuf>,
    /// The version of the game to check against when no game file is given. A .luma works on every version either way.
    #[arg(long, value_name = "ID", default_value = "RMGE01")]
    game_id: String,
    /// The game's own main.dol or disc, to compare a mod's changed program or whole changed disc with.
    #[arg(long, value_name = "FILE")]
    base: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    /// Say what a mod is and what would happen to it, without writing anything.
    Inspect {
        /// A mod folder, or a .zip, .7z or .rar file.
        input: PathBuf,
        #[command(flatten)]
        game: Game,
        /// Print the result as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Convert one mod to a .luma file.
    Convert {
        /// A mod folder, or a .zip, .7z or .rar file.
        input: PathBuf,
        /// The folder to write to.
        #[arg(short, long, value_name = "FOLDER", default_value = ".")]
        output: PathBuf,
        /// Build it even though files the game can't use are left out (they are listed first).
        #[arg(long)]
        accept_losses: bool,
        #[command(flatten)]
        game: Game,
    },
    /// Start a new mod: a project folder with a luma.json.
    New {
        /// The folder to create.
        folder: PathBuf,
        /// The mod's name; the folder's name when absent.
        #[arg(long)]
        name: Option<String>,
        /// Start with code: a plugin's source, and what builds it.
        #[arg(long)]
        plugin: bool,
    },
    /// Write the pixel program Nebula makes for each material of a model, to start a hand-written one from.
    Shader {
        /// A model (.bdl, .bmd, or a mod's .glb or .gltf), or an archive holding one.
        model: PathBuf,
        /// The folder to write to.
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
    },
    /// Build a project's plugin from source/*.c for every device the game runs on, with clang and lld.
    Build {
        /// A folder with a luma.json and a source folder.
        folder: PathBuf,
        /// clang, when it isn't on the PATH or in LLVM's usual place.
        #[arg(long, value_name = "FILE")]
        clang: Option<PathBuf>,
    },
    /// Build a project folder into a .luma file.
    Pack {
        /// A folder with a luma.json.
        folder: PathBuf,
        /// The folder to write to.
        #[arg(short, long, value_name = "FOLDER", default_value = ".")]
        output: PathBuf,
        #[command(flatten)]
        game: Game,
    },
    /// List every problem with a project folder or .luma file before you publish it.
    Check {
        /// A folder with a luma.json, or a .luma file.
        input: PathBuf,
        #[command(flatten)]
        game: Game,
    },
    /// Open a .luma file into a folder, to see how it's made.
    Unpack {
        /// A .luma file.
        input: PathBuf,
        /// The folder to open it into.
        #[arg(short, long, value_name = "FOLDER")]
        output: PathBuf,
    },
    /// Convert one file on its own, to test a model, texture or music track.
    Cook {
        /// The file.
        input: PathBuf,
        /// Where it goes in the game, such as ObjectData/Kuribo.arc.
        #[arg(long = "as", value_name = "PATH")]
        path: String,
        /// The file to write.
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,
        #[command(flatten)]
        game: Game,
    },
    /// List what the game's names are: settings, player (its numbers), objects, rules, tables (every table of your
    /// imported game with its rows and columns: what a merge/ file can add to, remove from or change), or messages (every
    /// message id of a language with its words, tags in brackets, as a "text" row takes them), or models (every model of
    /// your imported game with its joints, materials, textures and the animations beside it: what a model made like it,
    /// or in its place, is matched by).
    List {
        /// settings, player, objects, rules, tables, messages, models
        what: String,
        /// Only names holding this (for tables: a path or a column name).
        #[arg(long)]
        filter: Option<String>,
        /// Your imported game (Nebula's game.pak), for tables, messages and models.
        #[arg(long, value_name = "FILE")]
        game: Option<PathBuf>,
        /// The language folder of the messages: UsEnglish (the default), UsFrench, EuGerman, JpJapanese...
        #[arg(long, default_value = "UsEnglish")]
        language: String,
    },
    /// List the files of your imported game.
    Files {
        /// A folder of the game, such as ObjectData or StageData/EggStarGalaxy.
        folder: String,
        /// Your imported game (Nebula's game.pak).
        #[arg(long, value_name = "FILE")]
        game: PathBuf,
        /// Only paths holding this.
        #[arg(long)]
        filter: Option<String>,
    },
    /// Copy a file or a folder out of your imported game. Tables come as JSON too.
    Extract {
        /// Its place in the game, such as StageData/EggStarGalaxy.
        path: String,
        /// Your imported game (Nebula's game.pak).
        #[arg(long, value_name = "FILE")]
        game: PathBuf,
        /// The folder to write to.
        #[arg(short, long, value_name = "FOLDER", default_value = ".")]
        output: PathBuf,
    },
    /// Open one archive (.arc, .szs) from a disc or a mod into a folder. Tables come as JSON too.
    Unarc {
        /// The archive.
        input: PathBuf,
        /// The folder to write to.
        #[arg(short, long, value_name = "FOLDER")]
        output: PathBuf,
    },
    /// Show a table (.bcsv, .pa, .tbl, or a stage's placement file) as the JSON a mod writes.
    Table {
        /// The file.
        input: PathBuf,
        /// The file to write; the screen when absent.
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    /// Build a project and open it in Nebula.
    Run {
        /// A folder with a luma.json.
        folder: PathBuf,
        /// The game: Nebula.exe. Looked for in the usual places when absent.
        #[arg(long, value_name = "FILE")]
        nebula: Option<PathBuf>,
        #[command(flatten)]
        game: Game,
    },
    /// Convert every mod in a folder. One that fails doesn't stop the others.
    Batch {
        /// A folder of mods.
        folder: PathBuf,
        /// The folder to write to.
        #[arg(short, long, value_name = "FOLDER", default_value = ".")]
        output: PathBuf,
        /// Build it even though files the game can't use are left out (they are listed first).
        #[arg(long)]
        accept_losses: bool,
        #[command(flatten)]
        game: Game,
    },
}

/// Where Setup puts the game, on this device.
fn find_nebula() -> Option<PathBuf> {
    let name = if cfg!(windows) { "Nebula.exe" } else { "Nebula" };
    let mut places = Vec::new();
    if let Some(folder) = std::env::var_os("LOCALAPPDATA") {
        places.push(PathBuf::from(folder).join("Programs").join("Nebula").join(name));
    }
    if let Some(folder) = std::env::var_os("ProgramFiles") {
        places.push(PathBuf::from(folder).join("Nebula").join(name));
    }
    if let Some(folder) = std::env::var_os("HOME") {
        places.push(PathBuf::from(folder).join(".local").join("share").join("Nebula").join(name));
    }
    places.into_iter().find(|p| p.is_file())
}

fn open_store(game: &Game) -> Result<(Option<GameStore>, String)> {
    // --base: the game's own program, or its whole disc.
    if let Some(base) = &game.base {
        if base.extension().is_some_and(|e| e.eq_ignore_ascii_case("dol")) {
            plan::set_base_program(Some(base.clone()));
        } else {
            source::set_base_game(Some(base.clone()));
        }
    }
    match &game.game {
        Some(path) => {
            source::base_from_store(path);
            let store = GameStore::open(path)?;
            let id = store.game_id.clone();
            Ok((Some(store), id))
        }
        None => Ok((None, game.game_id.clone())),
    }
}

fn print_plan(name: &str, layout: &Layout, plan: &Plan) {
    println!("{name}");
    println!("  files: {}", plan.file_count());
    if let Some(pack) = &layout.pack {
        println!("  textures: {} ({} png, {} dds, {} mipmaps)", pack.textures, pack.png, pack.dds, pack.mips);
    }
    println!("  gameplay changes that work in Nebula: {}", plan.known_patches.len());
    println!("  code changes Nebula has no version of yet: {}", plan.unknown_code.len());
    for option in &plan.options {
        let names: Vec<&str> = option.choices.iter().map(|c| c.name.as_str()).collect();
        println!("  players choose {} in the game: {}", option.name, names.join(", "));
    }
    for layer in &plan.layers {
        if !layer.when.version.is_empty() {
            let versions: String = layer.when.version.iter().collect();
            println!("  {} files only for versions {versions}", layer.files.len());
        }
    }
    for note in &plan.notes {
        let label = match note.level {
            Level::Blocked => "CAN'T CONVERT",
            Level::Lost => "didn't make it",
            Level::Info => "good to know",
        };
        println!("  {label}: {}", note.text);
    }
}

fn convert_one(input: &Path, output: &Path, store: Option<&GameStore>, game_id: &str, accept_losses: bool) -> Result<package::Outcome> {
    let work = tempfile::tempdir().context("no temporary folder")?;
    let root = source::open(input, work.path())?;
    let layout = Layout::of(&root);
    let plan = plan::build(&layout, game_id, store)?;
    let name = source::mod_name(input);
    print_plan(&name, &layout, &plan);
    let outcome = package::write(&name, &layout, &plan, store, output, accept_losses)?;
    for (path, why) in &outcome.failed {
        println!("  left out: {path}: {why}");
    }
    println!("  {} of {} files, {} material programs", outcome.converted, outcome.total, outcome.programs);
    println!("  wrote {}", outcome.file.display());
    Ok(outcome)
}

fn run(cli: Cli) -> Result<ExitCode> {
    match cli.command {
        Command::Inspect { input, game, json } => {
            let (store, game_id) = open_store(&game)?;
            let work = tempfile::tempdir().context("no temporary folder")?;
            let root = source::open(&input, work.path())?;
            let layout = Layout::of(&root);
            let plan = plan::build(&layout, &game_id, store.as_ref())?;
            if json {
                println!("{}", serde_json::to_string_pretty(&serde_json::json!({ "layout": layout, "plan": plan }))?);
            } else {
                print_plan(&source::mod_name(&input), &layout, &plan);
            }
            Ok(if plan.blocked() { ExitCode::from(2) } else { ExitCode::SUCCESS })
        }
        Command::Convert { input, output, game, accept_losses } => {
            let (store, game_id) = open_store(&game)?;
            convert_one(&input, &output, store.as_ref(), &game_id, accept_losses)?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Shader { model, output } => {
            let written = project::write_generated_shaders(&model, &output)?;
            for name in &written {
                println!("{name}");
            }
            println!("Name one in a layer's materials, with custom-shaders in requires: see docs/format.md.");
            Ok(ExitCode::SUCCESS)
        }
        Command::New { folder, name, plugin } => {
            let name = name.unwrap_or_else(|| source::mod_name(&folder));
            project::new_project(&folder, &name)?;
            if plugin {
                project::add_plugin(&folder)?;
                println!("Its code is in source/plugin.c. Build it for every device with: lumalink build \"{}\"", folder.display());
            }
            println!("Started {name} in {}", folder.display());
            println!("Put files where they go in the game under layers/base/files/, and textures under layers/base/textures/.");
            println!("Then run: lumalink pack \"{}\"", folder.display());
            Ok(ExitCode::SUCCESS)
        }
        Command::Build { folder, clang } => {
            for library in project::build_plugin(&folder, clang.as_deref())? {
                println!("{}: {}", library.target, library.file);
            }
            println!("luma.json names each library. Then run: lumalink pack \"{}\"", folder.display());
            Ok(ExitCode::SUCCESS)
        }
        Command::Pack { folder, output, game } => {
            let (store, _) = open_store(&game)?;
            let outcome = project::pack(&folder, &output, store.as_ref())?;
            println!("{} files, {} material programs", outcome.converted, outcome.programs);
            println!("wrote {}", outcome.file.display());
            Ok(ExitCode::SUCCESS)
        }
        Command::Check { input, game } => {
            let (store, _) = open_store(&game)?;
            let findings = project::check(&input, store.as_ref())?;
            for found in &findings {
                println!("{}: {}", if found.serious { "PROBLEM" } else { "note" }, found.text);
            }
            let serious = findings.iter().filter(|f| f.serious).count();
            println!("{serious} problems, {} notes", findings.len() - serious);
            Ok(if serious == 0 { ExitCode::SUCCESS } else { ExitCode::from(2) })
        }
        Command::Unpack { input, output } => {
            project::unpack(&input, &output)?;
            println!("opened into {}", output.display());
            Ok(ExitCode::SUCCESS)
        }
        Command::Cook { input, path, output, game } => {
            let (store, game_id) = open_store(&game)?;
            let summary = project::cook_one(&input, &path, &game_id, store.as_ref().map(|s| s.path.as_path()), &output)?;
            println!("{summary}");
            println!("wrote {}", output.display());
            Ok(ExitCode::SUCCESS)
        }
        Command::List { what, filter, game, language } => {
            let keep = |name: &str| filter.as_deref().is_none_or(|f| name.to_ascii_lowercase().contains(&f.to_ascii_lowercase()));
            match what.as_str() {
                "settings" => {
                    for (name, default, words) in lumalink_core::inspect::SETTINGS.iter().filter(|s| keep(s.0)) {
                        println!("{name} ({default}): {words}");
                    }
                }
                "player" => {
                    for field in lumalink_core::player_constants::FIELDS.iter().filter(|f| keep(f.name)) {
                        if field.count > 1 {
                            println!("{}[{}]", field.name, field.count);
                        } else {
                            println!("{}", field.name);
                        }
                    }
                }
                "objects" => {
                    for (name, _, _) in lumalink_core::object_table::OBJECT_ROWS.iter().filter(|o| keep(o.0)) {
                        println!("{name}");
                    }
                }
                "rules" => {
                    for (name, words) in lumalink_core::inspect::STAGE_RULES.iter().filter(|s| keep(s.0)) {
                        println!("{name}: {words}");
                    }
                }
                "tables" => {
                    let game = game.context("tables are your game's: name its game.pak with --game")?;
                    let tables = lumalink_core::inspect::list_tables(&game, filter.as_deref())?;
                    for table in &tables {
                        let columns: Vec<String> = table.columns.iter().map(|(name, kind)| format!("{name}:{kind}")).collect();
                        println!("{} ({} rows): {}", table.path, table.rows, columns.join(" "));
                    }
                    println!("{} tables", tables.len());
                }
                "messages" => {
                    let game = game.context("messages are your game's: name its game.pak with --game")?;
                    let messages = lumalink_core::inspect::list_messages(&game, &language)?;
                    let mut shown = 0;
                    for (id, text, _) in &messages {
                        if keep(id) || keep(text) {
                            println!("{id}\t{}", text.replace('\n', "\\n"));
                            shown += 1;
                        }
                    }
                    let same = messages.iter().filter(|m| m.2).count();
                    println!("{shown} of {} messages; {same} read back as the game's own", messages.len());
                }
                "models" => {
                    let game = game.context("models are your game's: name its game.pak with --game")?;
                    let models = lumalink_core::inspect::list_models(&game, filter.as_deref())?;
                    for model in &models {
                        println!("{}", model.path);
                        println!("  joints: {}", model.names.joints.join(" "));
                        println!("  materials: {}", model.names.materials.join(" "));
                        let mut textures: Vec<&str> = Vec::new();
                        for texture in &model.names.textures {
                            if !textures.contains(&texture.as_str()) {
                                textures.push(texture);
                            }
                        }
                        println!("  textures: {}", textures.join(" "));
                        println!("  animations: {}", model.animations.join(" "));
                    }
                    println!("{} models", models.len());
                }
                other => bail!("{other}: choose settings, player, objects, rules, tables, messages or models"),
            }
            Ok(ExitCode::SUCCESS)
        }
        Command::Files { folder, game, filter } => {
            for (path, size) in lumalink_core::inspect::list_files(&game, &folder)? {
                if filter.as_deref().is_none_or(|f| path.to_ascii_lowercase().contains(&f.to_ascii_lowercase())) {
                    println!("{folder}/{path}  ({size} bytes)");
                }
            }
            Ok(ExitCode::SUCCESS)
        }
        Command::Extract { path, game, output } => {
            let written = lumalink_core::inspect::extract(&game, &path, &output)?;
            println!("{written} files written to {}", output.display());
            Ok(ExitCode::SUCCESS)
        }
        Command::Unarc { input, output } => {
            let written = lumalink_core::inspect::unarchive(&input, &output)?;
            println!("{written} files written to {}", output.display());
            Ok(ExitCode::SUCCESS)
        }
        Command::Table { input, output } => {
            let bytes = std::fs::read(&input).with_context(|| format!("{} cannot be read", input.display()))?;
            let json = serde_json::to_string_pretty(&lumalink_core::inspect::table_to_json(&bytes)?)?;
            match output {
                Some(file) => std::fs::write(file, json)?,
                None => println!("{json}"),
            }
            Ok(ExitCode::SUCCESS)
        }
        Command::Run { folder, nebula, game } => {
            let (store, _) = open_store(&game)?;
            let output = std::env::temp_dir().join("lumalink-run");
            std::fs::create_dir_all(&output)?;
            let outcome = project::pack(&folder, &output, store.as_ref())?;
            println!("built {}", outcome.file.display());
            let nebula = match nebula {
                Some(path) => path,
                None => find_nebula().context("Nebula.exe wasn't found: name it with --nebula")?,
            };
            std::process::Command::new(&nebula).arg("--open-mod").arg(&outcome.file).spawn().with_context(|| format!("{} can't be started", nebula.display()))?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Batch { folder, output, game, accept_losses } => {
            let (store, game_id) = open_store(&game)?;
            let mut inputs: Vec<PathBuf> = std::fs::read_dir(&folder)
                .with_context(|| format!("{} cannot be read", folder.display()))?
                .filter_map(std::result::Result::ok)
                .map(|e| e.path())
                .filter(|p| p.is_dir() || source::is_archive(p))
                .collect();
            inputs.sort();
            if inputs.is_empty() {
                bail!("{} holds no mods", folder.display());
            }
            let mut failed = 0;
            for input in &inputs {
                if let Err(error) = convert_one(input, &output, store.as_ref(), &game_id, accept_losses) {
                    failed += 1;
                    println!("  FAILED: {error:#}");
                }
                println!();
            }
            println!("{} of {} mods converted", inputs.len() - failed, inputs.len());
            Ok(if failed == 0 { ExitCode::SUCCESS } else { ExitCode::from(2) })
        }
    }
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}
