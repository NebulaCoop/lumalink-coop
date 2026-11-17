<#
.SYNOPSIS
Builds LumaLink-Setup.exe: LumaLink's command line and window, the C compiler mods are built with, and LumaKit, in
one signed program.

.DESCRIPTION
Builds lumalink and lumalink-gui (release), lays out the installed files in a temporary folder, builds
lumalink-setup with them packed in (crates/lumalink-setup/build.rs reads LUMALINK_SETUP_PAYLOAD), and signs the Setup
with the release key (installer/sign-release.ps1). LumaLink's updates run a downloaded Setup only when that signature
verifies against installer/release-public-key.xml and the version inside it is the release's.

The installed files:
  lumalink.exe, lumalink-gui.exe
  toolchain\bin\clang.exe, lld.exe      LLVM -Llvm (the version lumalink-core's build::LLVM names); Setup adds lld's
                                        other names (ld.lld.exe, lld-link.exe, ld64.lld.exe) as links
  toolchain\lib\clang\<major>\include   clang's own headers
  lumakit\                              LumaKit's source (Nebula ships it built, as lumakit.luma)
  test\                                 the test kit `lumalink test` runs (the game ships no test parts): Nebula's
                                        runner, its test extension and the game half's source, built from nebula\ by
                                        its tools\test_driver\test_kit.py
  editors\lumascript.vsix               the LumaScript extension for Visual Studio Code (installer/vsix.py), which
                                        Settings > Add LumaScript to Visual Studio Code installs
  licenses\                             LLVM's licence, the window font's and the Rust crates' (rust-crates.txt,
                                        installer/rust_licenses.py)
  LICENSE, README.md

Output: <OutputDirectory>\LumaLink-Setup.exe and LumaLink-Setup.exe.sig (LumaLink-Beta-Setup.exe for a beta) on x64,
LumaLink-Setup-windows-arm64.exe on Windows on ARM (update::package_name), the Windows files a release carries. The release workflow builds the Linux and macOS Setups (installer/build-setup.sh);
installer/sign-release.ps1 signs them.

Requires Rust (stable, MSVC target), Visual Studio 2022 with the C++ tools, Python 3, LLVM -Llvm installed, and nebula\
(the Nebula tree at nebula.commit).
The programs keep no build machine's folders (their panic texts name sources as cargo\..., rustup\..., lumalink\...).
#>
[CmdletBinding()]
param(
    [string]$Llvm = 'C:\Program Files\LLVM',
    [string]$OutputDirectory = '',
    # The release key's private half (RSA XML). The default is its usual place; -SigningKey '' builds an unsigned
    # Setup, which updates refuse.
    [string]$SigningKey = (Join-Path $env:USERPROFILE '.nebula-release-key\nebula-release-private.xml')
)
$ErrorActionPreference = 'Stop'
$installer = Split-Path -Parent $PSCommandPath
$repo = Split-Path -Parent $installer
if (-not $OutputDirectory) { $OutputDirectory = Join-Path $repo 'dist' }
$stage = Join-Path ([IO.Path]::GetTempPath()) ('lumalink-setup-' + [Guid]::NewGuid().ToString('N'))

function Invoke-Checked([string]$File, [string[]]$Arguments) {
    & $File @Arguments
    if ($LASTEXITCODE -ne 0) { throw "$File failed with exit code $LASTEXITCODE" }
}

function Copy-Into([string]$Source, [string]$Target) {
    New-Item -ItemType Directory -Force (Split-Path -Parent $Target) | Out-Null
    Copy-Item -LiteralPath $Source -Destination $Target
}

# Windows on ARM builds the ARM64 LumaLink (its own Setup name); everything else is x64.
$arm = $env:PROCESSOR_ARCHITECTURE -eq 'ARM64'
try {
    # The LLVM LumaLink builds mods with, and no other: a mod's code is the same bytes on every PC.
    $pinned = (Select-String -LiteralPath (Join-Path $repo 'crates\lumalink-core\src\build.rs') -Pattern '^pub const LLVM: &str = "([0-9.]+)";').Matches[0].Groups[1].Value
    $clang = Join-Path $Llvm 'bin\clang.exe'
    $lld = Join-Path $Llvm 'bin\lld.exe'
    foreach ($file in $clang, $lld) { if (-not (Test-Path -LiteralPath $file)) { throw "$file is not there: install LLVM $pinned" } }
    $found = (& $clang --version | Select-Object -First 1)
    if ($found -notmatch "clang version $([regex]::Escape($pinned))\b") { throw "$clang is '$found'; LumaLink builds with LLVM $pinned" }
    $major = $pinned.Split('.')[0]
    $headers = Join-Path $Llvm "lib\clang\$major\include"
    if (-not (Test-Path -LiteralPath $headers)) { throw "$headers is not there" }

    $version = (Select-String -LiteralPath (Join-Path $repo 'Cargo.toml') -Pattern '^version = "([^"]+)"').Matches[0].Groups[1].Value
    Write-Host "LumaLink $version with LLVM $pinned"

    # The source paths the programs keep (panic texts) name no build machine's folders.
    $cargoHome = if ($env:CARGO_HOME) { $env:CARGO_HOME } else { Join-Path $env:USERPROFILE '.cargo' }
    $rustupHome = if ($env:RUSTUP_HOME) { $env:RUSTUP_HOME } else { Join-Path $env:USERPROFILE '.rustup' }
    $nebula = (Get-Item -LiteralPath (Join-Path $repo 'nebula')).FullName
    $target = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $repo 'target' }
    $remaps = @(@($cargoHome, 'cargo'), @($rustupHome, 'rustup'), @($nebula, 'nebula'), @($repo, 'lumalink'))
    $env:CARGO_ENCODED_RUSTFLAGS = ($remaps | ForEach-Object { "--remap-path-prefix=$($_[0])=$($_[1])" }) -join [char]0x1f
    Push-Location $repo
    try { Invoke-Checked cargo @('build', '--release', '--locked', '-p', 'lumalink', '-p', 'lumalink-gui') }
    finally { Pop-Location }
    $release = Join-Path $target 'release'

    New-Item -ItemType Directory -Force $stage | Out-Null
    foreach ($name in 'lumalink.exe', 'lumalink-gui.exe') { Copy-Into (Join-Path $release $name) (Join-Path $stage $name) }
    Copy-Into $clang (Join-Path $stage 'toolchain\bin\clang.exe')
    Copy-Into $lld (Join-Path $stage 'toolchain\bin\lld.exe')
    New-Item -ItemType Directory -Force (Join-Path $stage "toolchain\lib\clang\$major") | Out-Null
    Copy-Item -LiteralPath $headers -Destination (Join-Path $stage "toolchain\lib\clang\$major\include") -Recurse
    Copy-Item -LiteralPath (Join-Path $repo 'lumakit') -Destination (Join-Path $stage 'lumakit') -Recurse
    # The test kit; on x64 its extension is built with the C++ library and Windows SDK of Visual Studio's x64
    # environment (the runner drives Windows x64 and Linux x64 only: on ARM the kit is the runner alone).
    $kitScript = Join-Path $nebula 'tools\test_driver\test_kit.py'
    if (-not (Test-Path -LiteralPath $kitScript)) { throw "$kitScript is not there: nebula\ must be the Nebula tree at nebula.commit" }
    if (-not $arm) { . (Join-Path $nebula 'tools\enter_msvc_environment.ps1') }
    Invoke-Checked python @($kitScript, '--out', (Join-Path $stage 'test'), '--cxx', (Join-Path $Llvm 'bin\clang++.exe'))
    Invoke-Checked python @((Join-Path $installer 'vsix.py'), (Join-Path $stage 'editors\lumascript.vsix'))
    Copy-Into (Join-Path $installer 'licenses\LLVM.txt') (Join-Path $stage 'licenses\LLVM.txt')
    Copy-Into (Join-Path $repo 'crates\lumalink-gui\assets\MPLUSRounded1c-OFL.txt') (Join-Path $stage 'licenses\M PLUS Rounded 1c.txt')
    foreach ($name in 'LICENSE', 'README.md') { Copy-Into (Join-Path $repo $name) (Join-Path $stage $name) }
    Invoke-Checked python @((Join-Path $installer 'rust_licenses.py'), $(if ($arm) { 'aarch64-pc-windows-msvc' } else { 'x86_64-pc-windows-msvc' }), (Join-Path $stage 'licenses\rust-crates.txt'), 'lumalink', 'lumalink-gui', 'lumalink-setup')

    # The Setup, with the staged files packed in (Zstandard 19: a minute or two).
    $env:LUMALINK_SETUP_PAYLOAD = $stage
    Push-Location $repo
    try { Invoke-Checked cargo @('build', '--release', '--locked', '-p', 'lumalink-setup') }
    finally { Pop-Location; Remove-Item Env:\LUMALINK_SETUP_PAYLOAD }

    New-Item -ItemType Directory -Force $OutputDirectory | Out-Null
    # A beta's Setup has its own name (update::package_name): a beta release never replaces the stable one's.
    $name = if ($version.ToLowerInvariant().Contains('-beta.')) { 'LumaLink-Beta-Setup' } else { 'LumaLink-Setup' }
    $name += if ($arm) { '-windows-arm64.exe' } else { '.exe' }
    $setup = Join-Path $OutputDirectory $name
    Copy-Item -LiteralPath (Join-Path $release 'LumaLink-Setup.exe') -Destination $setup -Force
    # Through a pipe: PowerShell runs a Windows (not console) program without one and would get nothing back.
    $marked = (& $setup --version | Out-String).Trim()
    if ($marked -ne "LumaLink Setup $version") { throw "The Setup says '$marked', not LumaLink Setup $version" }

    Remove-Item -LiteralPath "$setup.sig" -ErrorAction SilentlyContinue
    if ($SigningKey -and (Test-Path -LiteralPath $SigningKey)) {
        & (Join-Path $installer 'sign-release.ps1') -Files $setup -Version $version -SigningKey $SigningKey
    } elseif ($SigningKey) {
        Write-Warning "No release key at $SigningKey`: the Setup is unsigned and updates will not run it."
    }
    $item = Get-Item -LiteralPath $setup
    Write-Host ('{0}  {1:N1} MB  SHA-256 {2}' -f $item.FullName, ($item.Length / 1MB), (Get-FileHash -LiteralPath $setup -Algorithm SHA256).Hash)
}
finally {
    Remove-Item Env:\CARGO_ENCODED_RUSTFLAGS -ErrorAction SilentlyContinue
    if (Test-Path -LiteralPath $stage) { Remove-Item -LiteralPath $stage -Recurse -Force }
}
