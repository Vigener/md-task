use clap::{Parser, Subcommand};
use std::fs::OpenOptions; // ファイルを開くためのモジュール
use std::io::Write; // ファイルに書き込むためのモジュール

/// A simple CLI tool to manage tasks in a markdown file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new task
    Add {
        /// The content of the task
        task: String,
    },
    /// List all tasks
    List,
}

fn main() -> std::io::Result<()> { // このmain関数は、成功したら、何も返さず、失敗した場合はI/O関連のエラーを返す という意味
    // コマンドライン引数を解析する
    let cli = Cli::parse();

    // 解析結果（どのコマンドが呼ばれたか）に応じて処理を分岐
    match cli.command {
        Commands::Add { task } => {
            // "add" コマンドが呼ばれた場合の処理
            // ファイル書き込み処理
            // 1. ファイルを開くための設定
            let mut file = OpenOptions::new()
                .append(true) // 追記モードで開く
                .create(true) // ファイルが存在しない場合は新規作成
                .open("tasks.md")?; // "tasks.md"というファイルを開く [?]はエラーが発生した場合にそのエラーを返す
            // 2. タスクをファイルに書き込む
            writeln!(&mut file, "- [ ] {}", task)?; // タスクをマークダウン形式で書き込む
            // 3. 成功メッセージを表示
            println!("Task added: {}", task);
            // TODO: ここに実際にファイルに書き込む処理を追加していく
        }
        Commands::List => {
            // "list" コマンドが呼ばれた場合の処理
            println!("Listing all tasks...");
            // TODO: ここに実際にファイルから読み込む処理を追加していく
        }
    }
    Ok(())
}