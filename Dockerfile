# docker.io/library/ を追記して、イメージの完全な場所を指定
FROM docker.io/library/rust:1-slim-bookworm

# --- ▼▼▼ ここからロケール設定を追加 ▼▼▼ ---
# locales パッケージをインストール
RUN apt-get update && apt-get install -y locales

# 日本語ロケールを有効化
RUN sed -i 's/# ja_JP.UTF-8/ja_JP.UTF-8/' /etc/locale.gen && \
    locale-gen

# 環境変数を設定
ENV LANG=ja_JP.UTF-8
ENV LANGUAGE=ja_JP.UTF-8
ENV LC_ALL=ja_JP.UTF-8
# --- ▲▲▲ ここまでロケール設定を追加 ▲▲▲ ---

# 関連するコマンドを一つのRUN命令にまとめる
RUN apt-get update && apt-get install -y zsh git curl make \
    # Oh My Zshを非対話形式でインストールし、成功した場合のみ次に進む
    && sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh) --unattended" \
    # プラグインをクローン
    && git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions \
    && git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting \
    # .zshrcを編集してプラグインを有効化
    && sed -i 's/plugins=(git)/plugins=(git zsh-autosuggestions zsh-syntax-highlighting)/' ~/.zshrc \
    # キャッシュをクリーンアップしてイメージサイズを削減
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# コンテナ内での作業ディレクトリを指定
WORKDIR /usr/src/app

# Rustの開発ツールをインストール
RUN rustup component add rustfmt clippy

# このコマンドでコンテナを起動し続ける (開発用に重要)
CMD ["tail", "-f", "/dev/null"]
