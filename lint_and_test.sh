#!/bin/bash
set -e

# Colors for prettier output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m' # No Color

echo -e "${BOLD}===== Overpunch_ng Quality Check Suite =====${NC}"

# Check if rustup is installed
if ! command -v rustup &> /dev/null; then
    echo -e "${RED}Error: rustup is not installed. Please install rustup and try again.${NC}"
    exit 1
fi

# Check if required components are installed
for component in clippy rustfmt; do
    if ! rustup component list --installed | grep -q "$component"; then
        echo -e "${YELLOW}Installing $component...${NC}"
        rustup component add "$component"
    fi
done

# Run format check
echo -e "\n${BLUE}${BOLD}Running format check...${NC}"
if cargo fmt --all -- --check; then
    echo -e "${GREEN}✓ Format check passed${NC}"
else
    echo -e "${RED}✗ Format check failed. Run 'cargo fmt --all' to fix.${NC}"
    exit 1
fi

# Run clippy
echo -e "\n${BLUE}${BOLD}Running Clippy lints...${NC}"
if cargo clippy --all-targets --all-features -- -D warnings; then
    echo -e "${GREEN}✓ Clippy check passed${NC}"
else
    echo -e "${RED}✗ Clippy check failed. Please fix the warnings above.${NC}"
    exit 1
fi

# Run tests
echo -e "\n${BLUE}${BOLD}Running unit and integration tests...${NC}"
if RUST_BACKTRACE=1 cargo test; then
    echo -e "${GREEN}✓ All tests passed${NC}"
else
    echo -e "${RED}✗ Some tests failed. Please fix the failing tests above.${NC}"
    exit 1
fi

# Run property tests specifically
echo -e "\n${BLUE}${BOLD}Running property tests...${NC}"
if RUST_BACKTRACE=1 cargo test --test property_tests; then
    echo -e "${GREEN}✓ Property tests passed${NC}"
else
    echo -e "${RED}✗ Some property tests failed. Please fix the failing tests above.${NC}"
    exit 1
fi

# Run benchmarks
echo -e "\n${BLUE}${BOLD}Running benchmarks...${NC}"
if cargo bench -- --test; then
    echo -e "${GREEN}✓ Benchmarks passed${NC}"
else
    echo -e "${YELLOW}⚠ Benchmark test mode failed. This might be normal if benchmarks aren't designed for test mode.${NC}"
fi

echo -e "\n${GREEN}${BOLD}All quality checks passed!${NC}"
