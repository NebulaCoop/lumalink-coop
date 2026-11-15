//! The built Setup as LumaLink's updates and a terminal see it.

/// The version written into the Setup's bytes is the one it installs, and `--version` prints it.
#[cfg(windows)]
#[test]
fn the_setup_carries_its_version() {
    let exe = env!("CARGO_BIN_EXE_LumaLink-Setup");
    let bytes = std::fs::read(exe).unwrap();
    assert_eq!(lumalink_core::update::setup_version(&bytes).as_deref(), Some(lumalink_core::update::VERSION));
    let out = std::process::Command::new(exe).arg("--version").output().unwrap();
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), format!("LumaLink Setup {}", lumalink_core::update::VERSION));
}

/// A wrong command line is refused with exit code 2 and the options named.
#[cfg(windows)]
#[test]
fn a_wrong_option_is_refused() {
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_LumaLink-Setup")).args(["--frobnicate", "--language-is-not-here"]).env("LUMALINK_LANGUAGE", "UsEnglish").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    let said = String::from_utf8_lossy(&out.stderr);
    assert!(said.contains("--frobnicate") && said.contains("--silent"), "{said}");
}
