#!/bin/sh
# Builds the Linux or macOS LumaLink Setup on that system: LumaLink's command line and window, the C compiler mods are
# built with, and LumaKit, in one program (LumaLink-Setup-<platform>, LumaLink-Beta-Setup-<platform> for a beta).
#
#   installer/build-setup.sh --llvm <folder> [--output <folder>]
#
# --llvm is LLVM's release archive for this system, unpacked (the version lumalink-core's build::LLVM names): its
# clang, lld and clang's own headers are installed with LumaLink. --output defaults to dist/.
#
# The installed files (crates/lumalink-setup/build.rs packs them from LUMALINK_SETUP_PAYLOAD):
#   lumalink, lumalink-gui
#   toolchain/bin/clang, lld             Setup adds lld's other names (ld.lld, ld64.lld, lld-link) as links
#   toolchain/lib/clang/<major>/include  clang's own headers
#   lumakit/                             LumaKit's source (Nebula ships it built, as lumakit.luma)
#   test/                                the test kit `lumalink test` runs (the game ships no test parts): Nebula's
#                                        runner, and on Linux x64 its test extension and the game half's source, built
#                                        from nebula/ by its tools/test_driver/test_kit.py
#   editors/lumascript.vsix              the LumaScript extension for Visual Studio Code (installer/vsix.py)
#   licenses/                            LLVM's licence, the window font's and the Rust crates'
#   LICENSE, README.md
#
# The Setup is not signed here: the release key stays on the release PC, where installer/sign-release.ps1 signs it.
# Updates do not run an unsigned Setup.
#
# Requires Rust (stable), Python 3, nebula/ (the Nebula tree at nebula.commit), and on Linux a C compiler and the C++
# library's headers. The programs keep no build machine's folders (their panic texts name sources as cargo/...,
# rustup/..., lumalink/...).
set -eu

llvm=
output=
while [ $# -gt 0 ]; do
    case $1 in
        --llvm) llvm=${2:?--llvm needs a folder}; shift 2 ;;
        --output) output=${2:?--output needs a folder}; shift 2 ;;
        *) echo "build-setup.sh: unknown argument $1" >&2; exit 2 ;;
    esac
done
[ -n "$llvm" ] || { echo "build-setup.sh: --llvm <folder> is required" >&2; exit 2; }

repo=$(cd "$(dirname "$0")/.." && pwd)
llvm=$(cd "$llvm" && pwd)
[ -n "$output" ] || output=$repo/dist
mkdir -p "$output"
output=$(cd "$output" && pwd)

case $(uname -s)-$(uname -m) in
    Linux-x86_64) platform=linux-x64 ;;
    Linux-aarch64) platform=linux-arm64 ;;
    Darwin-arm64) platform=macos-arm64 ;;
    Darwin-x86_64) platform=macos-x64 ;;
    *) echo "build-setup.sh: $(uname -s) $(uname -m) has no LumaLink Setup" >&2; exit 1 ;;
esac
target=$(rustc -vV | sed -n 's/^host: //p')

# The LLVM LumaLink builds mods with, and no other: a mod's code is the same bytes on every PC.
# (tr: a Windows checkout's files may end their lines with CR LF.)
pinned=$(tr -d '\r' < "$repo/crates/lumalink-core/src/build.rs" | sed -n 's/^pub const LLVM: &str = "\([0-9.]*\)";/\1/p')
major=${pinned%%.*}
clang=$llvm/bin/clang-$major
[ -f "$clang" ] || clang=$llvm/bin/clang
for file in "$clang" "$llvm/bin/lld" "$llvm/lib/clang/$major/include/stddef.h"; do
    [ -f "$file" ] || { echo "build-setup.sh: $file is not there: unpack LLVM $pinned into $llvm" >&2; exit 1; }
done
"$clang" --version | grep -qE "clang version $pinned([^0-9]|\$)" || {
    echo "build-setup.sh: $clang is not LLVM $pinned" >&2; exit 1
}

version=$(tr -d '\r' < "$repo/Cargo.toml" | sed -n 's/^version = "\([^"]*\)"/\1/p' | head -n 1)
case $(echo "$version" | tr 'A-Z' 'a-z') in
    *-beta.*) name=LumaLink-Beta-Setup-$platform ;;
    *) name=LumaLink-Setup-$platform ;;
esac
echo "LumaLink $version ($platform) with LLVM $pinned"

release=${CARGO_TARGET_DIR:-$repo/target}/release
stage=$(mktemp -d "${TMPDIR:-/tmp}/lumalink-setup.XXXXXX")
trap 'rm -rf "$stage"' EXIT INT TERM

nebula=$(cd "$repo/nebula" && pwd -P)
[ -f "$nebula/tools/test_driver/test_kit.py" ] || {
    echo "build-setup.sh: $nebula/tools/test_driver/test_kit.py is not there: nebula/ must be the Nebula tree at nebula.commit" >&2; exit 1
}
# The source paths the programs keep (panic texts) name no build machine's folders. (0x1f: CARGO_ENCODED_RUSTFLAGS'
# separator, so a folder may hold spaces.)
sep=$(printf '\037')
CARGO_ENCODED_RUSTFLAGS="--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo$sep--remap-path-prefix=${RUSTUP_HOME:-$HOME/.rustup}=rustup$sep--remap-path-prefix=$nebula=nebula$sep--remap-path-prefix=$repo=lumalink"
export CARGO_ENCODED_RUSTFLAGS

cd "$repo"
cargo build --release --locked -p lumalink -p lumalink-gui

mkdir -p "$stage/toolchain/bin" "$stage/toolchain/lib/clang/$major" "$stage/licenses"
cp "$release/lumalink" "$release/lumalink-gui" "$repo/LICENSE" "$repo/README.md" "$stage/"
cp "$clang" "$stage/toolchain/bin/clang"
cp "$llvm/bin/lld" "$stage/toolchain/bin/lld"
cp -R "$llvm/lib/clang/$major/include" "$stage/toolchain/lib/clang/$major/include"
cp -R "$repo/lumakit" "$stage/lumakit"
python3 "$nebula/tools/test_driver/test_kit.py" --out "$stage/test" --cxx "$llvm/bin/clang++"
python3 "$repo/installer/vsix.py" "$stage/editors/lumascript.vsix"
cp "$repo/installer/licenses/LLVM.txt" "$stage/licenses/LLVM.txt"
cp "$repo/crates/lumalink-gui/assets/MPLUSRounded1c-OFL.txt" "$stage/licenses/M PLUS Rounded 1c.txt"
python3 "$repo/installer/rust_licenses.py" "$target" "$stage/licenses/rust-crates.txt" lumalink lumalink-gui lumalink-setup
chmod 755 "$stage/lumalink" "$stage/lumalink-gui" "$stage/toolchain/bin/clang" "$stage/toolchain/bin/lld"

# The Setup, with the staged files packed in (Zstandard 19: a minute or two).
LUMALINK_SETUP_PAYLOAD=$stage cargo build --release --locked -p lumalink-setup

setup=$output/$name
rm -f "$setup" "$setup.sig"
cp "$release/LumaLink-Setup" "$setup"
chmod 755 "$setup"
marked=$("$setup" --version)
[ "$marked" = "LumaLink Setup $version" ] || { echo "build-setup.sh: the Setup says '$marked', not LumaLink Setup $version" >&2; exit 1; }
if command -v sha256sum >/dev/null; then sum=$(sha256sum "$setup"); else sum=$(shasum -a 256 "$setup"); fi
echo "$setup  $(wc -c < "$setup" | tr -d ' ') bytes  SHA-256 ${sum%% *}"
