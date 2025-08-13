#!/bin/bash

echo "=== md-task Installation Setup ==="

# バイナリがインストールされているか確認
if ! command -v md-task &> /dev/null; then
    echo "Error: md-task binary not found. Please install it first:"
    echo "  cargo install --path ."
    echo "  or"
    echo "  cargo install md-task"
    exit 1
fi

echo "✅ md-task binary found"

# グローバル設定をインストール
echo "Installing global configuration..."
md-task config install

# 設定状況を表示
echo ""
md-task config status

echo ""
echo "=== Installation Complete! ==="
echo ""
echo "Next steps:"
echo "1. Create a project-specific config (optional):"
echo "   md-task config init"
echo ""
echo "2. Start adding tasks:"
echo "   md-task add \"My first task\""
echo ""
echo "3. View tasks:"
echo "   md-task list"
echo ""
echo "For more help:"
echo "   md-task --help"