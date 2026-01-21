# Script to apply remaining fiber architecture patches

function Extract-AndApply-Patch {
    param(
        [string]$PatchFile,
        [string]$TargetFile
    )
    
    Write-Host "Processing: $PatchFile -> $TargetFile"
    
    # Read patch file
    $content = Get-Content $PatchFile -Raw
    
    # Extract the target file path from patch
    if ($content -match '\+\+\+ b/(.+)') {
        $extractedPath = $matches[1]
        # Convert Unix path to Windows
        $extractedPath = $extractedPath -replace '/', '\'
        $fullPath = "D:\Documents\GitHub\WGPUI\$extractedPath"
        
        Write-Host "  Target: $fullPath"
        
        # For complete file replacements, extract the new content
        $lines = $content -split "`n"
        $output = @()
        $inContent = $false
        $inDeletion = $false
        
        foreach ($line in $lines) {
            # Start of actual diff content
            if ($line -match '^\+\+\+ b/') {
                $inContent = $true
                continue
            }
            
            # Process diff lines
            if ($inContent) {
                # Skip patch end marker
                if ($line -match '^\*\*\* End Patch') {
                    break
                }
                
                # Lines starting with + are additions
                if ($line -match '^\+' -and $line -notmatch '^\+\+\+') {
                    $output += $line.Substring(1)
                }
                # Lines starting with space or - in context we might need
                elseif ($line -match '^[^-]') {
                    # Context lines (no + or -)
                    if ($line -notmatch '^@@' -and $line -notmatch '^---') {
                        # Skip if this looks like a line number marker
                        if ($line -notmatch '^\d+[,\d]*\s') {
                            $output += $line
                        }
                    }
                }
            }
        }
        
        if ($output.Count -gt 0) {
            # Create directory if needed
            $dir = Split-Path $fullPath -Parent
            if (!(Test-Path $dir)) {
                New-Item -ItemType Directory -Path $dir -Force | Out-Null
            }
            
            # Write output
            $output -join "`n" | Set-Content $fullPath -NoNewline
            Write-Host "  ✓ Applied ($($output.Count) lines)" -ForegroundColor Green
            return $true
        } else {
            Write-Host "  ✗ No content extracted" -ForegroundColor Yellow
            return $false
        }
    } else {
        Write-Host "  ✗ Could not find target path in patch" -ForegroundColor Red
        return $false
    }
}

# Remaining critical patches to apply
$patchesToApply = @(
    @{Patch=".\patches\window.rs.patch"; File="crates\gpui\src\window.rs"},
    @{Patch=".\patches\context.rs.patch"; File="crates\gpui\src\window\context.rs"},
    @{Patch=".\patches\div.rs.patch"; File="crates\gpui\src\elements\div.rs"},
    @{Patch=".\patches\view.rs.patch"; File="crates\gpui\src\view.rs"},
    @{Patch=".\patches\style.rs.patch"; File="crates\gpui\src\style.rs"},
    @{Patch=".\patches\taffy.rs.patch"; File="crates\gpui\src\taffy.rs"}
)

Write-Host "=== Applying Fiber Architecture Patches ===" -ForegroundColor Cyan
Write-Host ""

$successCount = 0
$failCount = 0

foreach ($patch in $patchesToApply) {
    if (Extract-AndApply-Patch -PatchFile $patch.Patch -TargetFile $patch.File) {
        $successCount++
    } else {
        $failCount++
    }
    Write-Host ""
}

Write-Host "=== Summary ===" -ForegroundColor Cyan
Write-Host "Success: $successCount" -ForegroundColor Green
Write-Host "Failed: $failCount" -ForegroundColor Red
