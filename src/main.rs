use clap::{Parser, Subcommand};

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

fn main() {
    // コマンドライン引数を解析する
    let cli = Cli::parse();

    // 解析結果（どのコマンドが呼ばれたか）に応じて処理を分岐
    match cli.command {
        Commands::Add { task } => {
            // "add" コマンドが呼ばれた場合の処理
            println!("Adding a new task: {}", task);
            // TODO: ここに実際にファイルに書き込む処理を追加していく
        }
        Commands::List => {
            // "list" コマンドが呼ばれた場合の処理
            println!("Listing all tasks...");
            // TODO: ここに実際にファイルから読み込む処理を追加していく
        }
    }
}