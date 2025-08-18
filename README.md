# md-task

コマンドラインでMarkdownファイル上のタスクを管理するためのシンプルなCLIツールです。

## 概要

このツールは、指定したMarkdownファイル内の特定のセクション（例: `## タスク一覧`）にあるチェックリストを管理します。RustとPodmanの学習を目的として開発されています。

## 主な機能

- `add <タスク内容>` (短縮: `a`): 新しいタスクをMarkdownファイルに追加します。
  - `add <タスク内容> --priority <優先度>` (短縮: `a <タスク内容> -p <優先度>`): 優先度付きでタスクを追加します。
  - 優先度: `high` (🔴), `medium` (🟡, デフォルト), `low` (🟢)
- `list` (短縮: `ls`): 未完了のタスクを一覧表示します。
  - `list --all` (短縮: `ls -a`): 完了済みタスクとアーカイブ済みタスクも含めて全てのタスクを表示します。
- `done <タスク番号>` (短縮: `d`): タスクを完了済みにする (`- [ ]` -> `- [x]`)
- `remove <タスク番号>` (短縮: `rm`): 未完了タスクを削除します。
- `archive <タスク番号>` (短縮: `arc`): 完了済みタスクをアーカイブセクションに移動します。

### コマンドエイリアス

効率的な操作のため、各コマンドには短縮形が用意されています：
- `add` → `a`
- `list` → `ls`
- `done` → `d`
- `remove` → `rm`
- `archive` → `arc`

例: `md-task a "重要なタスク" -p high` や `md-task arc 1` のように使用できます。

## インストール

### 1. Rustバイナリのインストール

```bash
# Cargoからローカルインストール
cargo install --path .

# または将来的にcrates.ioから
cargo install md-task
```

### 2. 初期設定の実行

```bash
# 自動セットアップスクリプトを実行
./install.sh

# または手動で設定
md-task config install
```

### 3. 設定の確認

```bash
# 設定状況の確認
md-task config status

# 設定ファイルの場所確認
md-task config path
```

## 使用方法

### 基本的な使い方

```bash
# タスクを追加
md-task add "重要な会議の準備"

# 優先度付きでタスクを追加
md-task add "緊急対応" --priority high

# タスク一覧を表示
md-task list

# 完了済みタスクも含めて表示
md-task list --all

# タスクを完了済みにする
md-task done 1

# タスクを削除
md-task remove 2

# 完了済みタスクをアーカイブ
md-task archive 1
```

### デバッグ・開発用オプション

```bash
# 詳細な出力を表示
md-task --verbose list

# 環境変数でデバッグモードを有効化
export MD_TASK_VERBOSE=1
md-task add "デバッグテスト"

# 設定状況の詳細表示
md-task config status
```

## 設定ファイル

### 設定ファイルの優先順位

1. **ローカル設定** (`./md-task.toml`): プロジェクト固有の設定
2. **プロジェクト設定** (`$(project_root)/md-task.toml`): プロジェクト共通の設定
3. **グローバル設定** (`~/.config/md-task/config.toml`): ユーザー全体の設定

ローカル設定がグローバル設定を上書きします。

### 設定コマンド

```bash
# グローバル設定のインストール（初回のみ）
md-task config install

# ローカル設定ファイルを作成
md-task config init

# 現在の設定を表示
md-task config show

# 設定ファイルの場所を確認
md-task config path

# 設定状況の詳細表示
md-task config status
```

### 設定例

```toml
[task_management]
default_priority = "medium"          # デフォルトの優先度
auto_format = true                   # 自動ファイル形式正規化
allow_incomplete_in_archive = false  # アーカイブ内の未完了タスクを許可

[display]
show_completed_by_default = false    # list コマンドで完了済みタスクも表示

[file_paths]
task_file = "tasks.md"              # タスクファイルのパス
```

## 開発環境

### 開発中のテスト

```bash
# 開発中は cargo run を使用（インストール不要）
cargo run -- add "開発中のテスト"
cargo run -- list

# バイナリの再インストール
make dev-install
# または
cargo install --path . --force
```

### 開発用コマンド

```bash
# 開発環境のセットアップ
make setup

# 開発環境のリセット
make reset

# サイレントモードのテスト
make test-silent

# Verboseモードのテスト
make test-verbose

# クリーンアップ
make clean
```

## テストの実行

```bash
# すべてのテストを実行
cargo test

# 特定のテストのみ実行
cargo test config  # 設定関連のテストのみ実行
cargo test task    # タスク管理関連のテストのみ実行

# テスト実行時の詳細出力を見る
cargo test -- --nocapture
```

### テスト環境を充実させることのメリット

- 機能変更後も既存機能が壊れていないことを確認できる
- 新機能開発時にテストを先に書くことで要件を明確にできる（TDD）
- バグ修正時に再発防止のためのテストを追加できる

### 環境変数

- `MD_TASK_DEV=1`: 開発モード（`./dev-config/config.toml`を使用）
- `MD_TASK_VERBOSE=1`: 詳細出力モード
- `MD_TASK_DEBUG=1`: デバッグモード（verboseと同等）

## ファイル構造

### プロジェクト構造

```
src/
├── main.rs          # エントリーポイント、CLI定義
├── config.rs        # 設定管理（Config構造体、読み込み）
└── task.rs          # タスク操作（ファイル正規化、追加）
```

### 設定ファイル

```
# 開発環境
./dev-config/config.toml     # 開発用グローバル設定
./md-task.toml              # ローカル設定

# 本番環境
~/.config/md-task/config.toml   # ユーザーグローバル設定
./md-task.toml                  # プロジェクト設定
```

## 今後の機能（ロードマップ）

- [x] **基本的なタスク管理機能**
- [x] **`list`コマンドの強化**
- [x] **タスク管理機能の強化**
    - [x] **優先度の指定機能**: high/medium/low の3段階で優先度を設定
    - [x] **`archive <タスク番号>`**: 完了済みタスクをアーカイブセクションに移動する
- [x] **ファイル形式管理**
    - [x] **設定ファイル(`md-task.toml`)の導入**
    - [x] **階層的設定システム**: グローバル→プロジェクト→ローカルの優先順位
    - [x] **自動ファイル形式正規化**: 不要な空行削除、優先度自動付与など
- [x] **インストールとセットアップ**
    - [x] **自動インストールスクリプト**: グローバル設定の自動作成
    - [x] **設定状況確認機能**: `config status`コマンド
    - [x] **開発環境管理**: Makefile、環境変数による開発モード
- [x] **ユーザビリティの向上**
    - [x] **サイレントモード**: 通常使用時はデバッグメッセージを非表示
    - [x] **Verboseモード**: `--verbose`フラグや環境変数でデバッグ情報表示
    - [x] **モジュール分割**: config.rs、task.rsによるコード整理
- [ ] **さらなる機能強化**
    - [ ] **`archive --all`**: 完了済みタスクを一括アーカイブ
    - [ ] **優先度によるソート機能**: `list --sort-priority` で優先度順に表示
    - [ ] **期日機能**: `add "タスク" --due 2024-12-31` のような期日設定
    - [ ] **タグ機能**: `add "タスク" --tag work` のようなタグ付け
    - [ ] **パッケージ配布**: crates.ioへの公開
    - [ ] **シェル補完**: bash/zsh/fishでのタブ補完機能
    - [ ] **統計機能**: 完了率、期限切れタスクの統計表示

## 更新履歴

### v0.1.0 (2024-08-14)

- 基本的なタスク管理機能の実装
- 優先度機能（high/medium/low）の追加
- 階層的設定システムの導入
- 自動ファイル形式正規化機能
- 開発環境の整備（Makefile、環境変数管理）
- サイレント/Verboseモードの実装
- モジュール分割によるコード整理

## 勉強用メモ

### 開発環境向上ガイド（初心者向け）

ここでは、md-taskプロジェクトのための開発環境改善について、各項目ごとにどこにどのファイルを配置し、どのように使うかを説明します。

### 1. テスト環境の充実

#### ファイルの配置

1. **プロジェクトルートに `tests` ディレクトリを作成**
2. その中に以下のファイルを配置:
   - `tests/config_test.rs` - 設定関連のテスト
   - `tests/task_test.rs` - タスク管理機能のテスト

#### 使い方

```bash
# すべてのテストを実行
cargo test

# 特定のテストのみ実行
cargo test config  # 設定関連のテストのみ実行
cargo test task    # タスク管理関連のテストのみ実行

# テスト実行時の詳細出力を見る
cargo test -- --nocapture
```

#### メリット

- 機能変更後も既存機能が壊れていないことを確認できる
- 新機能開発時にテストを先に書くことで要件を明確にできる（TDD）
- バグ修正時に再発防止のためのテストを追加できる

### 2. CI/CD パイプラインの導入

#### ファイルの配置

1. **プロジェクトルートに `.github/workflows` ディレクトリを作成**
2. その中に以下のファイルを配置:
   - `.github/workflows/rust.yml` - テスト・ビルド・リントを自動実行
   - `.github/workflows/release.yml` - リリース時のビルド自動化

#### 使い方

特別な操作は不要です。GitHub上で以下のタイミングで自動実行されます：

- `rust.yml`: mainブランチへのプッシュ時やプルリクエスト作成/更新時
- `release.yml`: `v*` (例: v0.1.0)形式のタグをプッシュした時

リリースを作成する場合:

```bash
# タグを作成してプッシュ
git tag v0.1.0
git push origin v0.1.0
```

#### メリット

- コードの変更が自動的にテストされる
- フォーマットやリントエラーを早期に発見できる
- リリース作業が自動化され、ミスを防止できる

### 3. コード品質管理ツールの設定

#### ファイルの配置

プロジェクトルートに以下のファイルを配置:
- `rustfmt.toml` - Rustコードフォーマッタの設定
- `.clippy.toml` - Rustの静的解析ツールClippyの設定

#### 使い方

```bash
# コードのフォーマット
cargo fmt

# コードの静的解析
cargo clippy

# ビルド前に両方実行
cargo fmt && cargo clippy && cargo build
```

Makefileに追加すると便利:

```bash
# フォーマット
make format

# 静的解析
make lint
```

#### メリット

- コードスタイルの一貫性が保たれる
- 潜在的なバグやパフォーマンス問題を早期発見できる
- コードレビューが効率化される

### 4. 効率的な開発環境の設定

#### ファイルの配置

1. **VS Code設定**
   - `.vscode/settings.json` - プロジェクト固有のエディタ設定
   - `.vscode/extensions.json` - 推奨拡張機能リスト

2. **開発コンテナ設定**
   - `.devcontainer/Dockerfile` - 開発環境のコンテナ定義
   - `.devcontainer/devcontainer.json` - 開発コンテナの設定

3. **GitHub Copilot用の指示ファイル**
   - `.vscode/copilot-chat/instructions.md` - AIアシスタントへの指示

#### 使い方

**VS Code設定**:
- VS Codeでプロジェクトを開くと自動的に設定が適用されます
- 推奨拡張機能のインストールを促すポップアップが表示されます

**開発コンテナ**:
1. VS Codeに「Remote - Containers」拡張機能をインストール
2. コマンドパレット(F1)から「Remote-Containers: Open Folder in Container」を選択
3. プロジェクトフォルダを選択

**Copilot指示ファイル**:
- GitHub Copilot Chat拡張機能がインストールされていれば自動的に読み込まれます
- Copilotとのチャットでプロジェクト固有の知識が反映されます

#### メリット

- チーム全体で一貫した開発環境を共有できる
- 新メンバーが素早く開発を始められる
- 環境依存の問題を減らせる

### 実践的な使い方：開発サイクル例

初心者向けの典型的な開発フローは次のようになります：

1. **機能開発前**:
   ```bash
   git checkout -b feature/new-feature  # 新しいブランチを作成
   ```

2. **テストを書く**:
   ```bash
   # tests/new_feature_test.rs を作成
   cargo test  # テストを実行（この時点では失敗するはず）
   ```

3. **機能を実装**:
   ```bash
   # 機能を実装
   cargo fmt  # コードをフォーマット
   cargo clippy  # 静的解析
   cargo test  # テストが通るか確認
   ```

4. **コミットとプッシュ**:
   ```bash
   git add .
   git commit -m "feat: 新機能の実装"
   git push origin feature/new-feature
   ```

5. **GitHub上でプルリクエストを作成**
   - CI/CDが自動的に実行され、テストやリントが通るか確認されます

これらの開発環境改善により、コードの品質を維持しながら、効率的に開発を進めることができます。
