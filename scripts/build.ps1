# Local Windows development build: produce the Cargo artifact and the binary.
$ErrorActionPreference = 'Stop'

$Root = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
if ($Root.StartsWith('\\?\')) { $Root = $Root.Substring(4) }
$Cargo = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $Cargo) {
    [Console]::Error.WriteLine('dgr: Cargo is required to build this source')
    exit 1
}

Push-Location $Root
try {
    & cargo build --release --locked
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    $BinDir = Join-Path $Root 'bin'
    New-Item -ItemType Directory -Path $BinDir -Force | Out-Null
    Copy-Item -LiteralPath (Join-Path $Root 'target\release\dgr.exe') `
        -Destination (Join-Path $BinDir 'dgr.exe') -Force
} finally {
    Pop-Location
}
