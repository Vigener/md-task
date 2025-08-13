use clap::{Parser, Subcommand};
use std::io::{BufRead, BufReader}; // ファイルを読み込むためのモジュール
use std::fs::{File, OpenOptions}; // ファイルを開くためのモジュール
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
    #[command(alias = "a")]
    Add {
        /// The content of the task
        task: String,
        /// Priority level (high, medium, low)
        #[arg(short, long, default_value = "medium")]
        priority: String,
    },
    /// List all tasks
    #[command(alias = "ls")]
    List {
        /// Show all tasks including completed ones
        #[arg(short, long)]
        all: bool,
    },
    /// Mark a task as done
    #[command(alias = "d")]
    Done {
        /// The number of the task to mark as done
        task_number: usize,
    },
    /// Remove a task
    #[command(alias = "rm")]
    Remove {
        /// The number of the task to remove
        task_number: usize,
    },
}

fn main() -> std::io::Result<()> { // このmain関数は、成功したら、何も返さず、失敗した場合はI/O関連のエラーを返す という意味
    const TASK_FILE_PATH: &str = "tasks.md"; // タスクファイルのパスを定義
    // コマンドライン引数を解析する
    let cli = Cli::parse();

    // 解析結果（どのコマンドが呼ばれたか）に応じて処理を分岐
    match cli.command {
        Commands::Add { task, priority } => {
            // --- ファイル書き込み処理 ---

            // 1. 優先度の検証
            let valid_priorities = ["high", "medium", "low"];
            if !valid_priorities.contains(&priority.as_str()) {
                println!("ERROR: Invalid priority '{}'. Use: high, medium, or low", priority);
                return Ok(());
            }

            // 2. 優先度記号の設定
            let priority_symbol = match priority.as_str() {
                "high" => "🔴",
                "medium" => "🟡", 
                "low" => "🟢",
                _ => "🟡", // デフォルト
            };

            // 3. 既存ファイルの内容をチェック（改行で終わっているか確認）
            let needs_newline = if let Ok(contents) = std::fs::read_to_string(TASK_FILE_PATH) {
                !contents.is_empty() && !contents.ends_with('\n')
            } else {
                false // ファイルが存在しない場合は改行不要
            };

            // 4. ファイルを開くための設定
            let mut file = OpenOptions::new()
                .append(true) // 追記モードで開く
                .create(true) // ファイルが存在しない場合は新規作成
                .open(TASK_FILE_PATH)?; // "tasks.md"というファイルを開く [?]はエラーが発生した場合にそのエラーを返す
            
            // 5. 必要に応じて改行を追加してからタスクを書き込む
            if needs_newline {
                writeln!(&mut file)?; // 改行のみを追加
            }
            writeln!(&mut file, "- [ ] {} {}", priority_symbol, task)?; // タスクを優先度付きで書き込む
            
            // 6. 成功メッセージを表示
            println!("Task added: {} ({} priority)", task, priority);
        }
        Commands::List { all } => {
            // --- ファイル読み込み処理 ---

            // 1. ファイルを開く(task.md)
            let file = match File::open(TASK_FILE_PATH) { // 読み込み専用で開く
                Ok(file) => file, // ファイルが存在する場合はそのファイルを使用
                Err(_) => {
                    println!("No tasks found. Please add a task first.");
                    return Ok(()); // ファイルが存在しない場合は、何もせずに終了
                }
            };

            // 2. ファイルを1行ずつ読み込む
            let reader = BufReader::new(file); // ファイルをバッファリングして

            if all {
                // 全てのタスクを表示（完了済みも含む）
                println!("--- All Tasks ---");
                let mut incomplete_count = 0;
                let mut complete_count = 0;
                
                reader.lines()
                    .filter_map(Result::ok) // エラーのない行だけを取り出す
                    .filter(|line| line.starts_with("- [")) // チェックボックスのある行のみ
                    .for_each(|task_line| {
                        if task_line.starts_with("- [ ]") {
                            incomplete_count += 1;
                            let task_content = &task_line[6..]; // "- [ ] "を除去
                            println!("{}: {} (未完了)", incomplete_count, task_content);
                        } else if task_line.starts_with("- [x]") {
                            complete_count += 1;
                            let task_content = &task_line[6..]; // "- [x] "を除去
                            println!("✓: {} (完了済み)", task_content);
                        }
                    });
                
                println!("\n合計: 未完了 {}件, 完了済み {}件", incomplete_count, complete_count);
            } else {
                // 未完了タスクのみ表示（従来の動作）
                println!("--- Tasks ---");
                reader.lines()
                    .filter_map(Result::ok) // エラーのない行だけを取り出す
                    .filter(|line| line.starts_with("- [ ]")) // 未完了タスクのみをフィルタリング
                    .enumerate() // 行番号を付ける
                    .for_each(|(index, task_line)| {
                        let task_content = &task_line[6..]; // "- [ ] "を除去
                        println!("{}: {}", index + 1, task_content); // インデックスを1から始めて表示
                    });
            }
        }
        Commands::Done { task_number } => {
            // 1. ファイルを文字列として丸ごと読み込む
            let contents = std::fs::read_to_string(TASK_FILE_PATH)?;

            // 2. 未完了タスクを数えながら、指定された番号のタスクを書き換える
            let mut task_count = 0;
            let mut task_found = false;
            let new_contents: String = contents.lines()
                .map(|line| {
                    if line.starts_with("- [ ]") {
                        task_count += 1; // 未完了タスクのカウントを増やす
                        if task_count == task_number { // 指定されたタスク番号と一致する場合
                            task_found = true; // タスクが見つかったフラグを立てる
                            return line.replace("- [ ]", "- [x]"); // タスクを完了に変更
                        }
                    }
                    // 対象外の行はそのまま返す
                    line.to_string()
                })
                .collect::<Vec<String>>() // 変換結果をベクタに収集
                .join("\n"); // 行を改行で結合して新しい文字列

            // 3. 変更後の内容でファイルを上書き保存する
            if task_found {
                std::fs::write(TASK_FILE_PATH, new_contents)?; // ファイルに書き込む
                println!("Task {} marked as done.", task_number); // 成功メッセージ
                // TODO: 設定で`DONE`コマンドの最後に`list`コマンドを実行するようにするか選択できるようにする`
            } else {
                println!("ERROR: Task number {} not found.", task_number); // タスクが見つからなかった場合のメッセージ
            }
        }
        Commands::Remove { task_number } => {
            // 1. ファイルを文字列として丸ごと読み込む
            let contents = std::fs::read_to_string(TASK_FILE_PATH)?;

            // 2. 未完了タスクを数えながら、指定された番号のタスクを削除する
            let mut task_count = 0;
            let new_contents: String = contents.lines()
                .filter_map(|line| {
                    if line.starts_with("- [ ]") {
                        task_count += 1; // 未完了タスクのカウントを増やす
                        if task_count == task_number { // 指定されたタスク番号と一致する場合
                            return None; // この行は削除するのでNoneを返す
                        }
                    }
                    Some(line.to_string()) // 対象外の行はそのまま返す
                })
                .collect::<Vec<String>>() // 変換結果をベクタに収集
                .join("\n"); // 行を改行で結合して新しい文字列

            // 3. 変更後の内容でファイルを上書き保存する
            std::fs::write(TASK_FILE_PATH, new_contents)?; // ファイルに書き込む
            println!("Task {} removed.", task_number); // 成功メッセージ
        }
    }
    Ok(())
}