#!/bin/bash

echo "=== Development Environment Reset ==="

# 環境変数をクリア
unset MD_TASK_VERBOSE
unset MD_TASK_DEBUG
unset MD_TASK_DEV

echo "✅ Environment variables cleared"

# バイナリを再インストール
echo "Reinstalling md-task binary..."
cargo uninstall md-task 2>/dev/null || true
cargo install --path .

echo "✅ Binary reinstalled"

# テスト実行
echo "Testing silent mode..."
md-task add "Reset test"
md-task list

echo "=== Reset complete ==="