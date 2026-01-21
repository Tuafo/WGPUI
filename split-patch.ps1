param(
    [Parameter(Mandatory=$true)]
    [string]$PatchFile,
    [Parameter(Mandatory=$true)]
    [string]$OutputDir
)

if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir | Out-Null
}

$lines = Get-Content $PatchFile
$currentPatch = @()
$currentFile = $null
$fileHeaderPattern = '^diff --git a/(.+?) b/'

$patches = @()

foreach ($line in $lines) {
    if ($line -match $fileHeaderPattern) {
        if ($currentPatch.Count -gt 0 -and $currentFile) {
            $patches += [PSCustomObject]@{ File = $currentFile; Patch = $currentPatch }
        }
        $currentFile = $Matches[1].Trim()
        $currentPatch = @($line)
    } else {
        if ($null -ne $currentPatch) {
            $currentPatch += $line
        }
    }
}
if ($currentPatch.Count -gt 0 -and $currentFile) {
    $patches += [PSCustomObject]@{ File = $currentFile; Patch = $currentPatch }
}

foreach ($patch in $patches) {
    $outName = [System.IO.Path]::GetFileName($patch.File) + '.patch'
    $outPath = Join-Path $OutputDir $outName
    Set-Content -Path $outPath -Value @("*** Begin Patch") -Encoding UTF8
    Add-Content -Path $outPath -Value $patch.Patch -Encoding UTF8
    Add-Content -Path $outPath -Value "*** End Patch" -Encoding UTF8
}

Write-Host "Split $($patches.Count) patches into $OutputDir"