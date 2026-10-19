param([string]$BuildDirectory = (Join-Path $PSScriptRoot '.build'))
$ErrorActionPreference = 'Stop'
$projectPath = $PSScriptRoot
$env:CARGO_HOME = Join-Path $BuildDirectory 'cargo-home'
$env:CARGO_TARGET_DIR = Join-Path $BuildDirectory 'target'
# Avoid embedding the builder's Windows username or source directory in EXE paths.
$env:CARGO_ENCODED_RUSTFLAGS = @(
    "--remap-path-prefix=$env:USERPROFILE=/user",
    "--remap-path-prefix=$projectPath=/src"
) -join [char]31
cargo build --release --locked --manifest-path (Join-Path $projectPath 'Cargo.toml')
if ($LASTEXITCODE -ne 0) { throw 'Rust build failed' }
Copy-Item -LiteralPath (Join-Path $env:CARGO_TARGET_DIR 'release/cpe-monitor.exe') -Destination (Join-Path $projectPath 'CPE-Monitor.exe') -Force
