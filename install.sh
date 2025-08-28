#!/usr/bin/env bash

set -e

BINARY_NAME="mvp"
PROJECT_DIR="mvp"
TARGET_PATH="target/release/$BINARY_NAME"
INSTALL_PATH="/usr/local/bin/$BINARY_NAME"

# Color definitions
GREEN="\033[1;32m"
RED="\033[1;31m"
YELLOW="\033[1;33m"
BLUE="\033[1;34m"
RESET="\033[0m"

# Install function
install() {
    echo -e "${BLUE}🚀 Building project...${RESET}"
    cargo build --release --manifest-path "$PROJECT_DIR/Cargo.toml"

    echo -e "${BLUE}📦 Installing to $INSTALL_PATH ...${RESET}"
    sudo cp "$TARGET_PATH" "$INSTALL_PATH"

    echo -e "${GREEN}✅ Installation completed: $INSTALL_PATH${RESET}"
}

# Uninstall function
uninstall() {
    if [ -f "$INSTALL_PATH" ]; then
        echo -e "${YELLOW}🗑️ Removing $INSTALL_PATH ...${RESET}"
        sudo rm -f "$INSTALL_PATH"
        echo -e "${GREEN}✅ Uninstallation completed${RESET}"
    else
        echo -e "${RED}⚠️ Binary not found: $INSTALL_PATH${RESET}"
    fi
}

# Argument parsing
case "$1" in
    install)
        install
        ;;
    uninstall)
        uninstall
        ;;
    *)
        echo -e "${YELLOW}Usage: $0 {install|uninstall}${RESET}"
        ;;
esac
