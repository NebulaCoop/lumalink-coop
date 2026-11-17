<#
.SYNOPSIS
Signs LumaLink Setups with the release key: each file gets a .sig beside it (RSA PKCS#1 v1.5, SHA-256, base64).

.DESCRIPTION
LumaLink's updates run a downloaded Setup only when its .sig verifies against installer/release-public-key.xml and the
version written into it (LUMALINK-SETUP-VERSION=<version>;) is the release's. The release key's private half stays on
this PC, so the Setups the release workflow builds are signed here:

  installer/sign-release.ps1 -Tag v1.2.3

downloads each Setup the release has without a .sig (gh, signed in to GitHub), checks that it carries the release's
version, signs it and uploads the .sig files. Setups already signed are left as they are, and it names the Setups the
release does not have yet: run it again once the workflow adds them. Until a Setup is signed, updates do not offer that
release on its system.

  installer/sign-release.ps1 -Files <setup> [<setup> ...] -Version 1.2.3

signs local files only (installer/build-setup.ps1 does this for LumaLink-Setup.exe).

Requires PowerShell 5.1 or later; -Tag also requires the GitHub CLI (gh).
#>
[CmdletBinding(DefaultParameterSetName = 'Release')]
param(
    [Parameter(Mandatory, ParameterSetName = 'Release')]
    [string]$Tag,
    [Parameter(ParameterSetName = 'Release')]
    [string]$Repository = 'keepvaibin/lumalink',
    [Parameter(Mandatory, ParameterSetName = 'Files')]
    [string[]]$Files,
    [Parameter(Mandatory, ParameterSetName = 'Files')]
    [string]$Version,
    # The release key's private half (RSA XML).
    [string]$SigningKey = (Join-Path $env:USERPROFILE '.nebula-release-key\nebula-release-private.xml')
)
$ErrorActionPreference = 'Stop'
$installer = Split-Path -Parent $PSCommandPath

function Invoke-Checked([string]$File, [string[]]$Arguments) {
    & $File @Arguments
    if ($LASTEXITCODE -ne 0) { throw "$File failed with exit code $LASTEXITCODE" }
}

function Sign-Setup([string]$Path, [string]$Version, [Security.Cryptography.RSACryptoServiceProvider]$Rsa) {
    $bytes = [IO.File]::ReadAllBytes($Path)
    # The mark is ASCII; Latin-1 keeps every byte as one character, so the search finds it wherever it is.
    $text = [Text.Encoding]::GetEncoding(28591).GetString($bytes)
    # As update::setup_version reads it: every mark names the same version.
    $found = @([regex]::Matches($text, 'LUMALINK-SETUP-VERSION=([0-9A-Za-z.-]{1,40});') | ForEach-Object { $_.Groups[1].Value } | Sort-Object -Unique)
    if ($found.Count -ne 1) { throw "$Path names $($found.Count) versions, not one: it is not a LumaLink Setup" }
    $found = $found[0]
    if ($found -ne $Version) { throw "$Path is LumaLink Setup $found, not $Version" }
    $signature = $Rsa.SignData($bytes, [Security.Cryptography.CryptoConfig]::MapNameToOID('SHA256'))
    [IO.File]::WriteAllText("$Path.sig", [Convert]::ToBase64String($signature))
    Write-Host ('Signed {0}  SHA-256 {1}' -f (Split-Path -Leaf $Path), (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash)
}

if (-not (Test-Path -LiteralPath $SigningKey)) { throw "No release key at $SigningKey" }
$publicKey = (Get-Content -LiteralPath (Join-Path $installer 'release-public-key.xml') -Raw).Trim()
$rsa = New-Object Security.Cryptography.RSACryptoServiceProvider
$rsa.PersistKeyInCsp = $false
try {
    $rsa.FromXmlString((Get-Content -LiteralPath $SigningKey -Raw))
    if ($rsa.ToXmlString($false) -ne $publicKey) { throw 'The signing key does not match installer/release-public-key.xml.' }

    if ($PSCmdlet.ParameterSetName -eq 'Files') {
        foreach ($file in $Files) { Sign-Setup (Resolve-Path -LiteralPath $file).Path $Version $rsa }
        return
    }

    $version = $Tag -replace '^v', ''
    $folder = Join-Path ([IO.Path]::GetTempPath()) ('lumalink-sign-' + [Guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory $folder | Out-Null
    try {
        # A release's Setups, as update::package_name names them (LumaLink-Beta-Setup... for a beta, update::is_beta).
        $setup = if ($version.ToLowerInvariant().Contains('-beta.')) { 'LumaLink-Beta-Setup' } else { 'LumaLink-Setup' }
        $expected = @("$setup.exe", "$setup-windows-arm64.exe", "$setup-linux-x64", "$setup-linux-arm64",
                      "$setup-macos-arm64", "$setup-macos-x64")
        $assets = @(Invoke-Checked gh @('release', 'view', $Tag, '--repo', $Repository, '--json', 'assets', '--jq', '.assets[].name'))
        $missing = @($expected | Where-Object { $assets -notcontains $_ })
        $signed = @($expected | Where-Object { ($assets -contains $_) -and ($assets -contains "$_.sig") })
        $names = @($expected | Where-Object { ($assets -contains $_) -and ($assets -notcontains "$_.sig") })
        foreach ($name in $signed) { Write-Host "$name is signed already." }
        if ($missing.Count) { Write-Host "Not on $Tag yet (run this again once the release workflow adds them): $($missing -join ', ')" }
        if (-not $names.Count) {
            Write-Host "Nothing to sign on $Tag."
            return
        }
        foreach ($name in $names) {
            Invoke-Checked gh @('release', 'download', $Tag, '--repo', $Repository, '--pattern', $name, '--dir', $folder)
            Sign-Setup (Join-Path $folder $name) $version $rsa
        }
        $signatures = @($names | ForEach-Object { Join-Path $folder "$_.sig" })
        Invoke-Checked gh (@('release', 'upload', $Tag, '--repo', $Repository, '--clobber') + $signatures)
        Write-Host "Uploaded $($signatures.Count) signatures to $Tag."
    }
    finally { Remove-Item -LiteralPath $folder -Recurse -Force -ErrorAction SilentlyContinue }
}
finally { $rsa.Dispose() }
