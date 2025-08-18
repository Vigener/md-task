.PHONY: install setup test clean dev-install reset lint format watch coverage audit security-check

# ローカルインストール（開発用）
install:
    cargo install --path .
    ./install.sh

# 開発環境のセットアップ
setup:
    export MD_TASK_DEV=1
    mkdir -p ./dev-config
    echo '[task_management]\ndefault_priority = "medium"\nauto_format = true\nallow_incomplete_in_archive = false\n\n[display]\nshow_completed_by_default = false\n\n[file_paths]\ntask_file = "tasks.md"' > ./dev-config/config.toml

# テスト実行
test:
    cargo test
    ./test-config.sh

# クリーンアップ
clean:
    cargo clean
    rm -f tasks.md local-tasks.md dev-tasks.md md-task.toml
    rm -rf dev-config/

# 開発環境リセット
reset:
    @echo "=== Resetting development environment ==="
    -unset MD_TASK_VERBOSE MD_TASK_DEBUG MD_TASK_DEV
    cargo uninstall md-task || true
    cargo install --path .
    @echo "✅ Reset complete"

# 開発用再インストール
dev-install:
    cargo install --path . --force

# サイレントテスト
test-silent:
    @echo "Testing silent mode..."
    @unset MD_TASK_VERBOSE && unset MD_TASK_DEBUG && cargo run -- add "Silent test"
    @unset MD_TASK_VERBOSE && unset MD_TASK_DEBUG && cargo run -- list

# デバッグテスト
test-verbose:
    @echo "Testing verbose mode..."
    MD_TASK_VERBOSE=1 cargo run -- add "Verbose test"
    MD_TASK_VERBOSE=1 cargo run -- list

# リント実行
lint:
    @echo "=== Running linting checks ==="
    cargo clippy -- -D warnings
    @echo "✅ Linting passed"

# フォーマット適用
format:
    @echo "=== Formatting code ==="
    cargo fmt --all
    @echo "✅ Formatting complete"

# テスト監視（ファイル変更時に自動テスト）
watch:
    @echo "=== Watching for changes and running tests ==="
    cargo watch -x 'test -- --nocapture'

# テストカバレッジ計測
coverage:
    @echo "=== Generating test coverage report ==="
    cargo tarpaulin --out Html
    @echo "✅ Coverage report generated"

# 依存関係の脆弱性チェック
audit:
    @echo "=== Checking dependencies for vulnerabilities ==="
    cargo audit
    @echo "✅ Security audit complete"

# ビルドとセキュリティチェックを実行
security-check: audit lint
    @echo "=== Security checks completed ==="
