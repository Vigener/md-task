use std::env;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

// md-taskのconfig.rsをテストするためにクレートを再エクスポート
#[path = "../src/config.rs"]
mod config;

#[test]
fn test_load_config_from_local_file() {
    // テスト用の一時ディレクトリを作成
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();

    // カレントディレクトリをテスト用ディレクトリに変更
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(temp_path).unwrap();

    // テスト用の設定ファイルを作成
    let config_content = r#"
[task_management]
default_priority = "high"
auto_format = false
allow_incomplete_in_archive = true

[display]
show_completed_by_default = true

[file_paths]
task_file = "test-tasks.md"
"#;

    fs::write("md-task.toml", config_content).unwrap();

    // 設定を読み込む
    let config = config::load_config();

    // 設定値を検証
    assert_eq!(config.task_management.default_priority, "high");
    assert!(!config.task_management.auto_format);
    assert!(config.task_management.allow_incomplete_in_archive);
    assert!(config.display.show_completed_by_default);
    assert_eq!(config.file_paths.task_file, "test-tasks.md");

    // テスト後に元のディレクトリに戻す
    env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_get_config_dir() {
    // 環境変数をテスト用に設定
    unsafe {
        env::set_var("MD_TASK_DEV", "1");
    }

    // 開発環境の設定ディレクトリをチェック
    let config_dir = config::get_config_dir();
    assert_eq!(config_dir, PathBuf::from("./dev-config"));

    // 環境変数をクリア
    unsafe {
        env::remove_var("MD_TASK_DEV");
    }
}
