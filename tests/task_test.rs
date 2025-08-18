use std::fs;
use std::io::Write;
use tempfile::tempdir;
use std::env;

// md-taskのconfig.rsとtask.rsをテストするためにクレートを再エクスポート
#[path = "../src/config.rs"]
mod config;
#[path = "../src/task.rs"]
mod task;

#[test]
fn test_add_task_to_file() {
    // テスト用の一時ディレクトリを作成
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    // テスト用のファイルパス
    let task_file = temp_path.join("test-tasks.md");
    let task_file_path = task_file.to_str().unwrap();
    
    // タスクを追加
    task::add_task_to_file(task_file_path, "テストタスク", "high").unwrap();
    
    // ファイルの内容を検証
    let content = fs::read_to_string(task_file_path).unwrap();
    assert!(content.contains("## タスク一覧"));
    assert!(content.contains("- [ ] 🔴 テストタスク"));
    
    // さらにタスクを追加
    task::add_task_to_file(task_file_path, "普通の優先度タスク", "medium").unwrap();
    
    // 再度ファイルの内容を検証
    let content = fs::read_to_string(task_file_path).unwrap();
    assert!(content.contains("- [ ] 🔴 テストタスク"));
    assert!(content.contains("- [ ] 🟡 普通の優先度タスク"));
}

#[test]
fn test_normalize_task_file() {
    // テスト用の一時ディレクトリを作成
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    // テスト用のファイルパス
    let task_file = temp_path.join("normalize-test.md");
    let task_file_path = task_file.to_str().unwrap();
    
    // 不正な形式のタスクファイルを作成
    let content = r#"
- [ ] 優先度なしタスク

- [x] 優先度なし完了タスク


- [ ] 連続した空行があるタスク


## アーカイブ
- [ ] アーカイブ内の未完了タスク
- [x] 🟡 アーカイブ内の完了タスク
"#;
    fs::write(task_file_path, content).unwrap();
    
    // デフォルト設定でファイルを正規化
    let config = config::Config::default();
    task::normalize_task_file(task_file_path, &config).unwrap();
    
    // 正規化後の内容を検証
    let normalized = fs::read_to_string(task_file_path).unwrap();
    
    // 1. 「## タスク一覧」セクションが追加されているか
    assert!(normalized.contains("## タスク一覧"));
    
    // 2. 連続した空行が1行になっているか
    assert!(!normalized.contains("\n\n\n"));
    
    // 3. 優先度記号がないタスクに medium 優先度が追加されているか
    assert!(normalized.contains("- [ ] 🟡 優先度なしタスク"));
    assert!(normalized.contains("- [x] 🟡 優先度なし完了タスク"));
    
    // 4. アーカイブ内の未完了タスクがタスク一覧に移動されているか
    let lines: Vec<&str> = normalized.lines().collect();
    let task_section_idx = lines.iter().position(|&l| l == "## タスク一覧").unwrap();
    let archive_section_idx = lines.iter().position(|&l| l == "## アーカイブ").unwrap();
    
    // タスク一覧セクションとアーカイブセクションの間に「アーカイブ内の未完了タスク」が移動されているか確認
    let task_moved = lines[task_section_idx..archive_section_idx]
        .iter()
        .any(|&l| l.contains("- [ ] 🟡 アーカイブ内の未完了タスク"));
    assert!(task_moved);
}