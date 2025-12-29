#!/bin/bash
# sysmap test script
# Run this from the sysmap project directory after building

set -e  # Exit on first error

EXE="./target/release/sysmap"

echo "========================================"
echo "sysmap Test Suite"
echo "========================================"
echo ""

# Check if executable exists
if [ ! -f "$EXE" ]; then
    echo "ERROR: $EXE not found. Run 'cargo build --release' first."
    exit 1
fi

echo "1. Version"
$EXE --version
echo ""

echo "2. Help"
$EXE --help
echo ""

echo "3. Init (force rebuild)"
$EXE init --force
echo ""

echo "4. Summary (human readable)"
$EXE summary
echo ""

echo "5. Summary (JSON)"
$EXE summary --json
echo ""

echo "6. Tree (default depth)"
$EXE tree
echo ""

echo "7. Tree (depth 2)"
$EXE tree -d 2
echo ""

echo "8. Tree (src/ only)"
$EXE tree src/
echo ""

echo "9. Find 'main'"
$EXE find main
echo ""

echo "10. Find rust files containing 'mod'"
$EXE find mod -t rs
echo ""

echo "11. Find by language (rust)"
$EXE find "" -l rust
echo ""

echo "12. Find by purpose (entry points)"
$EXE find "" -p entry
echo ""

echo "13. Find by purpose (modules)"
$EXE find "" -p module
echo ""

echo "14. Update"
$EXE update
echo ""

echo "15. Quiet mode test"
$EXE -q init --force
echo "(Should have no output above)"
echo ""

echo "16. No color mode (tree)"
$EXE --no-color tree -d 1
echo ""

echo "========================================"
echo "All tests completed!"
echo "========================================"
