mod config;
mod task;

use clap::{Parser, Subcommand};
use config::{load_config, show_config_paths, show_config_status};
use std::fs::File;
use std::io::{BufRead, BufReader};
use task::{add_task_to_file, archive_all_completed_tasks, normalize_task_file};

/// A simple CLI tool to manage tasks in a markdown file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
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
    /// Archive a completed task
    #[command(alias = "arc")]
    Archive {
        /// The number of the completed task to archive
        task_number: Option<usize>,
        /// Archive all completed tasks
        #[arg(short, long)]
        all: bool,
    },
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigAction {
    /// Install global configuration (run once after installation)
    Install,
    /// Create a local config file in current directory
    Init,
    /// Show current configuration
    Show,
    /// Show config file locations
    Path,
    /// Show comprehensive configuration status
    Status,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    // verboseフラグが指定された場合は環境変数を設定
    if cli.verbose {
        unsafe {
            std::env::set_var("MD_TASK_VERBOSE", "1");
        }
    }

    let config = load_config();
    let task_file_path = &config.file_paths.task_file;

    match cli.command {
        Commands::Add { task, priority } => {
            // 優先度の検証
            let valid_priorities = ["high", "medium", "low"];
            if !valid_priorities.contains(&priority.as_str()) {
                println!(
                    "ERROR: Invalid priority '{}'. Use: high, medium, or low",
                    priority
                );
                return Ok(());
            }

            add_task_to_file(task_file_path, &task, &priority)?;
            println!("Task added: {} ({} priority)", task, priority);
        }
        Commands::List { all } => {
            // --- ファイル読み込み処理 ---

            // 1. ファイルを開く(task.md)
            let file = match File::open(task_file_path) {
                // 読み込み専用で開く
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
                let mut archived_count = 0;
                let mut in_archive_section = false;

                reader.lines()
                    .map_while(Result::ok) // エラーのない行だけを取り出す
                    .for_each(|line| {
                        // アーカイブセクションの開始を検知
                        if line == "## アーカイブ" {
                            in_archive_section = true;
                            if incomplete_count > 0 || complete_count > 0 {
                                println!(); // セクション間に空行を追加
                            }
                            println!("--- アーカイブ済み ---");
                            return;
                        }

                        if line.starts_with("- [ ]") {
                            incomplete_count += 1;
                            let task_content = &line[6..]; // "- [ ] "を除去
                            println!("{}: {} (未完了)", incomplete_count, task_content);
                        } else if line.starts_with("- [x]") {
                            if in_archive_section {
                                archived_count += 1;
                                let task_content = &line[6..]; // "- [x] "を除去
                                println!("A{}: {} (アーカイブ済み)", archived_count, task_content);
                            } else {
                                complete_count += 1;
                                let task_content = &line[6..]; // "- [x] "を除去
                                println!("✓: {} (完了済み)", task_content);
                            }
                        }
                    });

                println!(
                    "\n合計: 未完了 {}件, 完了済み {}件, アーカイブ済み {}件",
                    incomplete_count, complete_count, archived_count
                );
            } else {
                // 未完了タスクのみ表示（従来の動作）
                println!("--- Tasks ---");
                reader.lines()
                    .map_while(Result::ok) // エラーのない行だけを取り出す
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
            let contents = std::fs::read_to_string(task_file_path)?;

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
                std::fs::write(task_file_path, new_contents)?; // ファイルに書き込む
                println!("Task {} marked as done.", task_number); // 成功メッセージ
            // TODO: 設定で`DONE`コマンドの最後に`list`コマンドを実行するようにするか選択できるようにする`
            } else {
                println!("ERROR: Task number {} not found.", task_number); // タスクが見つからなかった場合のメッセージ
            }
        }
        Commands::Remove { task_number } => {
            // 1. ファイルを文字列として丸ごと読み込む
            let contents = std::fs::read_to_string(task_file_path)?;

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
            std::fs::write(task_file_path, new_contents)?; // ファイルに書き込む
            println!("Task {} removed.", task_number); // 成功メッセージ
        }
        Commands::Archive { task_number, all } => {
            if all {
                // 全ての完了済みタスクをアーカイブ
                archive_all_completed_tasks(task_file_path)?;
                println!("All completed tasks have been archived.");
            } else if let Some(task_num) = task_number {
                // 指定された番号の完了済みタスクをアーカイブ
                // 1. ファイルを文字列として丸ごと読み込む
                let contents = std::fs::read_to_string(task_file_path)?;

                // 2. 完了済みタスクを数えながら、指定された番号のタスクを見つける
                let mut completed_task_count = 0;
                let mut task_found = false;
                let mut archived_task = String::new();
                let mut lines: Vec<String> = Vec::new();
                let mut archive_section_exists = false;
                let mut archive_section_start = 0;

                // まず、完了済みタスクを探してアーカイブ対象を特定
                for line in contents.lines() {
                    if line == "## アーカイブ" {
                        archive_section_exists = true;
                        archive_section_start = lines.len();
                    }

                    if line.starts_with("- [x]") {
                        completed_task_count += 1;
                        if completed_task_count == task_num {
                            task_found = true;
                            archived_task = line.to_string();
                            continue; // この行は除外
                        }
                    }
                    lines.push(line.to_string());
                }

                if !task_found {
                    println!("ERROR: Completed task number {} not found.", task_num);
                    return Ok(());
                }

                // 3. アーカイブセクションを追加または既存セクションに追記
                if !archive_section_exists {
                    // アーカイブセクションが存在しない場合は新しく作成
                    lines.push("".to_string()); // 空行
                    lines.push("## アーカイブ".to_string());
                    lines.push("".to_string()); // 空行
                    lines.push(archived_task);
                } else {
                    // 既存のアーカイブセクションに追記
                    lines.insert(archive_section_start + 1, "".to_string()); // セクションタイトルの後に空行
                    lines.insert(archive_section_start + 2, archived_task);
                }

                // 4. 変更後の内容でファイルを上書き保存
                let new_contents = lines.join("\n");
                std::fs::write(task_file_path, new_contents)?;
                println!("Task {} archived successfully.", task_num);
            } else {
                println!("ERROR: Please specify either --all or a task number.");
            }
        }
        Commands::Config { action } => match action {
            ConfigAction::Install => {
                if let Err(e) = config::install_global_config() {
                    println!("Error installing global config: {}", e);
                }
            }
            ConfigAction::Init => {
                if let Err(e) = config::create_local_config() {
                    println!("Error creating config file: {}", e);
                }
            }
            ConfigAction::Show => {
                println!("{:#?}", config);
            }
            ConfigAction::Path => {
                show_config_paths();
            }
            ConfigAction::Status => {
                show_config_status();
            }
        },
    }

    // 全てのコマンド実行後にファイル形式を正規化
    if config.task_management.auto_format {
        normalize_task_file(task_file_path, &config)?;
    }

    Ok(())
}
