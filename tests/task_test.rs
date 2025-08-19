use std::fs;
use tempfile::tempdir;

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

#[test]
fn test_archive_all_completed_tasks() {
    // テスト用の一時ディレクトリを作成
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();

    // テスト用のファイルパス
    let task_file = temp_path.join("archive-all-test.md");
    let task_file_path = task_file.to_str().unwrap();

    // 完了済みタスクと未完了タスクが混在するテストファイルを作成
    let content = r#"## タスク一覧

- [ ] 🔴 未完了の重要タスク
- [x] 🟡 完了済みタスク1
- [ ] 🟢 未完了の低優先度タスク
- [x] 🔴 完了済みタスク2
- [x] 🟢 完了済みタスク3

## アーカイブ

- [x] 🟡 既存のアーカイブタスク
"#;
    fs::write(task_file_path, content).unwrap();

    // archive_all_completed_tasks関数を実行
    task::archive_all_completed_tasks(task_file_path).unwrap();

    // 結果を検証
    let result = fs::read_to_string(task_file_path).unwrap();
    let lines: Vec<&str> = result.lines().collect();

    // 1. タスク一覧セクションには未完了タスクのみが残っているか
    let task_section_idx = lines.iter().position(|&l| l == "## タスク一覧").unwrap();
    let archive_section_idx = lines.iter().position(|&l| l == "## アーカイブ").unwrap();

    let task_section_lines: Vec<&str> = lines[task_section_idx..archive_section_idx]
        .iter()
        .filter(|&&l| l.starts_with("- "))
        .cloned()
        .collect();

    // タスク一覧には未完了タスクのみ
    assert_eq!(task_section_lines.len(), 2);
    assert!(task_section_lines.iter().all(|&l| l.starts_with("- [ ]")));
    assert!(
        task_section_lines
            .iter()
            .any(|&l| l.contains("未完了の重要タスク"))
    );
    assert!(
        task_section_lines
            .iter()
            .any(|&l| l.contains("未完了の低優先度タスク"))
    );

    // 2. アーカイブセクションに全ての完了済みタスクが移動されているか
    let archive_section_lines: Vec<&str> = lines[archive_section_idx..]
        .iter()
        .filter(|&&l| l.starts_with("- [x]"))
        .cloned()
        .collect();

    // アーカイブには4つの完了済みタスクがある（元の1つ + 移動した3つ）
    assert_eq!(archive_section_lines.len(), 4);
    assert!(
        archive_section_lines
            .iter()
            .any(|&l| l.contains("既存のアーカイブタスク"))
    );
    assert!(
        archive_section_lines
            .iter()
            .any(|&l| l.contains("完了済みタスク1"))
    );
    assert!(
        archive_section_lines
            .iter()
            .any(|&l| l.contains("完了済みタスク2"))
    );
    assert!(
        archive_section_lines
            .iter()
            .any(|&l| l.contains("完了済みタスク3"))
    );
}
