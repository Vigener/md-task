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
    Add {
        /// The content of the task
        task: String,
    },
    /// List all tasks
    List,
    Done {
        // The number of the task to mark as done
        task_number: usize,
    }
}

fn main() -> std::io::Result<()> { // このmain関数は、成功したら、何も返さず、失敗した場合はI/O関連のエラーを返す という意味
    // コマンドライン引数を解析する
    let cli = Cli::parse();

    // 解析結果（どのコマンドが呼ばれたか）に応じて処理を分岐
    match cli.command {
        Commands::Add { task } => {
            // --- ファイル書き込み処理 ---

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
            // --- ファイル読み込み処理 ---

            // 1. ファイルを開く(task.md)
            let file = match File::open("tasks.md") { // 読み込み専用で開く
                Ok(file) => file, // ファイルが存在する場合はそのファイルを使用
                Err(_) => {
                    println!("No tasks found. Please add a task first.");
                    return Ok(()); // ファイルが存在しない場合は、何もせずに終了
                }
            };

            // 2. ファイルを1行ずつ読み込む
            let reader = BufReader::new(file); // ファイルをバッファリングして

            println!("--- Tasks ---");
            // 3. 1行ずつ処理し、タスクだけをフィルタリングして表示
            reader.lines()
                .filter_map(Result::ok) // エラーのない行だけを取り出す
                .filter(|line| line.starts_with("- [ ]")) // 未完了タスクのみをフィルタリング
                .enumerate() // 行番号を付ける
                .for_each(|(index, task_line)| {
                    println!("{}: {}", index + 1, &task_line[6..]); // インデックスを1から始めて表示
                });
        }
        Commands::Done { task_number } => {
            // 1. ファイルを文字列として丸ごと読み込む
            let contents = std::fs::read_to_string("tasks.md")?;

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
                std::fs::write("tasks.md", new_contents)?; // ファイルに書き込む
                println!("Task {} marked as done.", task_number); // 成功メッセージ
                // TODO: 設定で`DONE`コマンドの最後に`list`コマンドを実行するようにするか選択できるようにする`
            } else {
                println!("ERROR: Task number {} not found.", task_number); // タスクが見つからなかった場合のメッセージ
            }
        }
    }
    Ok(())
}