#!/bin/bash

# Stop on error
set -e

# Colors
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}🚀 Building MiniGit in release mode...${NC}"
cargo build --release

# Path to compiled binary
BIN_PATH="target/release/minigit"
echo BIN_PATH=$BIN_PATH
# Check if build succeeded
if [ ! -f "$BIN_PATH" ]; then
    echo "❌ Build failed. Exiting."
    exit 1
fi

echo "✅ Build succeeded: $BIN_PATH"

# Install to /usr/local/bin
echo -e "${CYAN}📦 Installing mingit to /usr/local/bin...${NC}"
sudo cp "$BIN_PATH" /usr/local/bin/mingit
sudo chmod +x /usr/local/bin/mingit

# Check installed version
echo -e "${CYAN}✅ Installed successfully!${NC}"
echo -e "${GREEN}Version:$(/usr/local/bin/mingit --version)${NC}"