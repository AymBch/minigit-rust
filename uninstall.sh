#!/bin/bash

# Stop on error
set -e

# Colors
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}🧹 Uninstalling MiniGit...${NC}"

if [ -f "/usr/local/bin/mingit" ]; then
    sudo rm /usr/local/bin/mingit
    echo -e "${RED}❌ mingit removed from /usr/local/bin${NC}"
else
    echo -e "${RED}⚠️ mingit not found in /usr/local/bin${NC}"
fi

echo -e "${CYAN}✨ Uninstallation complete.${NC}"