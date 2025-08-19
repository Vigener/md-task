use crate::config::Config;

fn is_verbose() -> bool {
    std::env::var("MD_TASK_VERBOSE").is_ok() || std::env::var("MD_TASK_DEBUG").is_ok()
}

pub fn normalize_task_file(file_path: &str, config: &Config) -> std::io::Result<()> {
    let contents = match std::fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(_) => return Ok(()), // ファイルが存在しない場合は何もしない
    };

    let mut lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let mut modified = false;

    // 1. 先頭に「## タスク一覧」セクションがない場合は追加
    if lines.is_empty() || lines[0] != "## タスク一覧" {
        lines.insert(0, "".to_string());
        lines.insert(0, "## タスク一覧".to_string());
        modified = true;
    }

    // 2. 連続する空行を1行にまとめる
    let mut normalized_lines = Vec::new();
    let mut prev_empty = false;

    for line in lines {
        let is_empty = line.trim().is_empty();

        if is_empty && prev_empty {
            // 連続する空行はスキップ
            continue;
        }

        // 3. 優先度記号がないタスクに medium 優先度を追加
        if line.starts_with("- [ ]") || line.starts_with("- [x]") {
            let task_content = &line[6..];
            if !task_content.starts_with("🔴")
                && !task_content.starts_with("🟡")
                && !task_content.starts_with("🟢")
            {
                let new_line = format!("{} 🟡 {}", &line[..5], task_content);
                normalized_lines.push(new_line);
                modified = true;
            } else {
                normalized_lines.push(line);
            }
        } else {
            normalized_lines.push(line);
        }

        prev_empty = is_empty;
    }

    // 4. アーカイブ内の未完了タスクをタスク一覧に戻す（設定による）
    if !config.task_management.allow_incomplete_in_archive {
        let mut in_archive = false;
        let mut tasks_to_move = Vec::new();
        let mut final_lines = Vec::new();

        for line in normalized_lines {
            if line == "## アーカイブ" {
                in_archive = true;
                final_lines.push(line);
                continue;
            }

            if in_archive && line.starts_with("- [ ]") {
                // アーカイブ内の未完了タスクを移動対象に
                tasks_to_move.push(line);
                modified = true;
            } else {
                final_lines.push(line);
            }
        }

        // 移動するタスクをタスク一覧セクションに挿入
        if !tasks_to_move.is_empty() {
            let mut archive_index = final_lines.len();
            for (i, line) in final_lines.iter().enumerate() {
                if line == "## アーカイブ" {
                    archive_index = i;
                    break;
                }
            }

            // アーカイブセクションの前にタスクを挿入
            for (i, task) in tasks_to_move.into_iter().enumerate() {
                final_lines.insert(archive_index + i, task);
            }
        }

        normalized_lines = final_lines;
    }

    // 5. ファイル末尾の改行を確保
    if !normalized_lines.is_empty() && !normalized_lines.last().unwrap().is_empty() {
        normalized_lines.push("".to_string());
        modified = true;
    }

    // 変更があった場合のみファイルを更新
    if modified {
        let new_contents = normalized_lines.join("\n");
        std::fs::write(file_path, new_contents)?;
        if is_verbose() {
            println!("File format normalized.");
        }
    }

    Ok(())
}

pub fn add_task_to_file(file_path: &str, task: &str, priority: &str) -> std::io::Result<()> {
    // 優先度記号の設定
    let priority_symbol = match priority {
        "high" => "🔴",
        "medium" => "🟡",
        "low" => "🟢",
        _ => "🟡", // デフォルト
    };

    let new_task_line = format!("- [ ] {} {}", priority_symbol, task);

    if let Ok(contents) = std::fs::read_to_string(file_path) {
        // ファイルが存在する場合：適切な位置に挿入
        let mut lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
        let mut archive_section_index = None;

        // アーカイブセクションの位置を探す
        for (i, line) in lines.iter().enumerate() {
            if line == "## アーカイブ" {
                archive_section_index = Some(i);
                break;
            }
        }

        if let Some(index) = archive_section_index {
            // アーカイブセクションがある場合：その前に挿入
            lines.insert(index, new_task_line);
        } else {
            // アーカイブセクションがない場合：最後に追加
            lines.push(new_task_line);
        }

        // ファイルを上書き保存
        let new_contents = lines.join("\n");
        std::fs::write(file_path, new_contents)?;
    } else {
        // ファイルが存在しない場合：新規作成（## タスク一覧付き）
        let initial_content = format!("## タスク一覧\n\n{}", new_task_line);
        std::fs::write(file_path, initial_content)?;
    }

    Ok(())
}

pub fn archive_all_completed_tasks(file_path: &str) -> std::io::Result<()> {
    // ファイルを読み込む
    let contents = std::fs::read_to_string(file_path)?;

    let mut lines: Vec<String> = Vec::new();
    let mut completed_tasks: Vec<String> = Vec::new();
    let mut archive_section_exists = false;
    let mut archive_section_start = 0;

    // 1. 完了済みタスクを収集し、それらをメインセクションから除外
    for line in contents.lines() {
        if line == "## アーカイブ" {
            archive_section_exists = true;
            archive_section_start = lines.len();
            lines.push(line.to_string());
        } else if line.starts_with("- [x]") {
            // 完了済みタスクの場合
            if archive_section_exists {
                // すでにアーカイブセクション内の場合はそのまま保持
                lines.push(line.to_string());
            } else {
                // タスク一覧セクション内の場合はアーカイブ対象として収集
                completed_tasks.push(line.to_string());
            }
        } else {
            lines.push(line.to_string());
        }
    }

    // 2. 完了済みタスクがなければ何もしない
    if completed_tasks.is_empty() {
        if is_verbose() {
            println!("No completed tasks to archive.");
        }
        return Ok(());
    }

    // 3. アーカイブセクションがない場合は作成
    if !archive_section_exists {
        lines.push("".to_string()); // 空行
        lines.push("## アーカイブ".to_string());
        archive_section_start = lines.len() - 1;
    }

    // 4. 完了済みタスクをアーカイブセクションに追加
    let insert_position = archive_section_start + 1;
    for (i, task) in completed_tasks.iter().enumerate() {
        if i == 0 {
            lines.insert(insert_position, "".to_string()); // アーカイブセクションの後に空行
        }
        lines.insert(insert_position + i + 1, task.clone());
    }

    // 5. ファイルを更新
    let new_contents = lines.join("\n");
    std::fs::write(file_path, new_contents)?;

    if is_verbose() {
        println!("Archived {} completed tasks.", completed_tasks.len());
    }

    Ok(())
}
