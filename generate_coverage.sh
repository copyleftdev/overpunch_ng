#!/bin/bash
set -e

echo "===== Overpunch_ng Code Coverage Generator ====="

# Install required tools if not already installed
if ! command -v cargo-llvm-cov &> /dev/null; then
    echo "Installing cargo-llvm-cov..."
    cargo install cargo-llvm-cov
fi

echo "Generating code coverage report..."
cargo llvm-cov clean --workspace
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Check if HTML report generation is requested
if [ "$1" == "--html" ]; then
    if ! command -v genhtml &> /dev/null; then
        echo "Warning: genhtml not found. To install on Ubuntu, run: sudo apt-get install lcov"
        echo "Skipping HTML report generation."
    else
        echo "Generating HTML report..."
        genhtml -o coverage/ lcov.info
        echo "HTML report generated in coverage/ directory"
    fi
fi

echo "✓ Coverage data generated to lcov.info"
echo ""
echo "To upload to Codecov manually, install the Codecov uploader and run:"
echo "  codecov -f lcov.info -t <your-codecov-token>"
echo ""
echo "For HTML report, run this script with --html flag:"
echo "  ./generate_coverage.sh --html"
