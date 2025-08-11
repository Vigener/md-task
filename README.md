# md-task

コマンドラインでMarkdownファイル上のタスクを管理するためのシンプルなCLIツールです。

## 概要

このツールは、指定したMarkdownファイル内の特定のセクション（例: `## タスク`）にあるチェックリストを管理します。RustとPodmanの学習を目的として開発されています。

## 主な機能

- `add <タスク内容>`: 新しいタスクをMarkdownファイルに追加します。
- `list`: 現在のタスクを一覧表示します。

## 開発環境のセットアップ

このプロジェクトはPodmanを使ったコンテナ環境で開発します。

1. **イメージのビルド:**
   ```bash
   # DNSに問題がある場合は --dns=1.1.1.1 を追加
   podman build -t md-task-dev .
   ```
2. **コンテナの起動:**
   ```bash 
   # ホストの.zshrcをコンテナにマウントして、zshの設定を反映
    # DNSに問題がある場合は --dns=1.1.1.1 を追加
   podman run -d --name md-task-container --dns=1.1.1.1 -v .:/usr/src/app -v ~/.zshrc:/root/.zshrc:ro md-task-dev
   ```
3. **コンテナ内での作業
    ```bash
    podman exec -it md-task-container zsh
    ```
