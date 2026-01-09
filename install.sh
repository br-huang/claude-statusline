#!/bin/bash
# install.sh - Install Claude Statusline to Cluade Code

set -e

echo "🔨 Building release version..."
cargo build --release

echo "📁 Creating directory..."
mkdir -p ~/.claude/bin

echo "📦 Installing binary..."
cp target/release/claude-statusline ~/.claude/bin
chmod +x ~/.claude/bin/claude-statusline

echo "⚙️ Confiuring Claude Code..."
SETTINGS_FILE="$HOME/.claude/settings.json"

if [ -f "$SETTINGS_FILE" ]; then
    # Check if Claude Code is already configured
    if command -v jq &> /dev/null; then
        jq '.statusLine = {"type": "command", "command": "~/.claude/bin/claude-statusline"}' 
        "$SETTINGS_FILE" > tmp.json && mv tmp.json "$SETTINGS_FILE"
    else
        echo "⚠️ Please manually add statusline config to $SETTINGS_FILE"
    fi
else 
    # Create Claude Code settings file
    echo '{
    "statusLine": {
        "type": "command",
        "command": "~/.claude/bin/claude-statusline"
        },
    }' > "$SETTINGS_FILE"
fi

echo "✅ Installation complete!"
echo "🔄 Please restart Claude Code to see your new statusline/"