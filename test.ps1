# sysmap test script
# Run this from the sysmap project directory after building

$exe = ".\target\release\sysmap.exe"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "sysmap Test Suite" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if executable exists
if (-not (Test-Path $exe)) {
    Write-Host "ERROR: $exe not found. Run 'cargo build --release' first." -ForegroundColor Red
    exit 1
}

Write-Host "1. Version" -ForegroundColor Yellow
& $exe --version
Write-Host ""

Write-Host "2. Help" -ForegroundColor Yellow
& $exe --help
Write-Host ""

Write-Host "3. Init (force rebuild)" -ForegroundColor Yellow
& $exe init --force
Write-Host ""

Write-Host "4. Summary (human readable)" -ForegroundColor Yellow
& $exe summary
Write-Host ""

Write-Host "5. Summary (JSON)" -ForegroundColor Yellow
& $exe summary --json
Write-Host ""

Write-Host "6. Tree (default depth)" -ForegroundColor Yellow
& $exe tree
Write-Host ""

Write-Host "7. Tree (depth 2)" -ForegroundColor Yellow
& $exe tree -d 2
Write-Host ""

Write-Host "8. Tree (src/ only)" -ForegroundColor Yellow
& $exe tree src/
Write-Host ""

Write-Host "9. Find 'main'" -ForegroundColor Yellow
& $exe find main
Write-Host ""

Write-Host "10. Find rust files containing 'mod'" -ForegroundColor Yellow
& $exe find mod -t rs
Write-Host ""

Write-Host "11. Find by language (rust)" -ForegroundColor Yellow
& $exe find "" -l rust
Write-Host ""

Write-Host "12. Find by purpose (entry points)" -ForegroundColor Yellow
& $exe find "" -p entry
Write-Host ""

Write-Host "13. Find by purpose (modules)" -ForegroundColor Yellow
& $exe find "" -p module
Write-Host ""

Write-Host "14. Update" -ForegroundColor Yellow
& $exe update
Write-Host ""

Write-Host "15. Quiet mode test" -ForegroundColor Yellow
& $exe -q init --force
Write-Host "(Should have no output above)"
Write-Host ""

Write-Host "16. No color mode (tree)" -ForegroundColor Yellow
& $exe --no-color tree -d 1
Write-Host ""

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "All tests completed!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
