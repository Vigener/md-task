use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub task_management: TaskManagementConfig,
    pub display: DisplayConfig,
    pub file_paths: FilePathsConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskManagementConfig {
    pub default_priority: String,
    pub auto_format: bool,
    pub allow_incomplete_in_archive: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayConfig {
    pub show_completed_by_default: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilePathsConfig {
    pub task_file: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            task_management: TaskManagementConfig {
                default_priority: "medium".to_string(),
                auto_format: true,
                allow_incomplete_in_archive: false,
            },
            display: DisplayConfig {
                show_completed_by_default: false,
            },
            file_paths: FilePathsConfig {
                task_file: "tasks.md".to_string(),
            },
        }
    }
}

// 設定ディレクトリを取得（開発環境を考慮）
#[allow(dead_code)]
pub fn get_config_dir() -> PathBuf {
    // 1. 開発環境チェック（環境変数 MD_TASK_DEV が設定されている場合）
    if std::env::var("MD_TASK_DEV").is_ok() {
        return PathBuf::from("./dev-config");
    }

    // 2. プロダクション環境の設定
    if let Ok(config_dir) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(config_dir).join("md-task")
    } else if let Some(home_dir) = dirs::home_dir() {
        home_dir.join(".config").join("md-task")
    } else {
        // フォールバック: カレントディレクトリ
        PathBuf::from(".")
    }
}

#[allow(dead_code)]
fn load_config_from_file(path: &PathBuf) -> Option<Config> {
    if let Ok(contents) = std::fs::read_to_string(path) {
        toml::from_str(&contents).ok()
    } else {
        None
    }
}

#[allow(dead_code)]
fn get_config_search_paths() -> Vec<PathBuf> {
    let mut paths = vec![
        // 1. カレントディレクトリのローカル設定（最優先）
        PathBuf::from("md-task.toml"),
    ];

    // 2. プロジェクトルートを探す（存在する場合のみ追加）
    if let Some(project_root) = find_project_root() {
        let project_config = project_root.join("md-task.toml");
        if project_config != paths[0] {
            // 重複回避
            paths.push(project_config);
        }
    }

    // 3. グローバル設定
    paths.push(get_config_dir().join("config.toml"));

    paths
}

#[allow(dead_code)]
fn find_project_root() -> Option<PathBuf> {
    let mut current = std::env::current_dir().ok()?;

    loop {
        // Cargo.tomlまたは.gitがあるディレクトリをプロジェクトルートとみなす
        if current.join("Cargo.toml").exists() || current.join(".git").exists() {
            return Some(current);
        }

        if !current.pop() {
            break;
        }
    }
    None
}

#[allow(dead_code)]
fn is_verbose() -> bool {
    std::env::var("MD_TASK_VERBOSE").is_ok() || std::env::var("MD_TASK_DEBUG").is_ok()
}

#[allow(dead_code)]
pub fn load_config() -> Config {
    let mut config = Config::default();
    let search_paths = get_config_search_paths();

    // 設定ファイルを優先順で検索・読み込み（グローバル→プロジェクト→ローカルの順で上書き）
    for path in search_paths.iter().rev() {
        // 逆順で読み込み（後で上書き）
        if let Some(loaded_config) = load_config_from_file(path) {
            config = loaded_config;
            if is_verbose() {
                println!("Loaded config from: {}", path.display());
            }
        }
    }

    // ローカル設定があれば最終的に上書き
    let local_config_path = PathBuf::from("md-task.toml");
    if let Some(local_config) = load_config_from_file(&local_config_path) {
        merge_configs(&mut config, local_config);
        if is_verbose() {
            println!("Applied local config overrides");
        }
    }

    config
}

#[allow(dead_code)]
fn merge_configs(base: &mut Config, override_config: Config) {
    // 各フィールドをローカル設定で上書き
    base.task_management = override_config.task_management;
    base.display = override_config.display;
    base.file_paths = override_config.file_paths;
}

#[allow(dead_code)]
pub fn create_local_config() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    let toml_string = toml::to_string_pretty(&config)?;
    std::fs::write("md-task.toml", toml_string)?;
    println!("Created local config file: md-task.toml");
    Ok(())
}

#[allow(dead_code)]
pub fn install_global_config() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = get_config_dir();
    let config_file = config_dir.join("config.toml");

    // 設定ディレクトリを作成
    std::fs::create_dir_all(&config_dir)?;

    // グローバル設定ファイルが存在しない場合のみ作成
    if !config_file.exists() {
        let default_config = Config::default();
        let toml_string = toml::to_string_pretty(&default_config)?;
        std::fs::write(&config_file, toml_string)?;
        println!("Created global config file: {}", config_file.display());
    } else {
        println!(
            "Global config file already exists: {}",
            config_file.display()
        );
    }

    Ok(())
}

#[allow(dead_code)]
pub fn show_config_paths() {
    let search_paths = get_config_search_paths();

    println!("Configuration file search order:");
    for (i, path) in search_paths.iter().enumerate() {
        let exists = path.exists();
        println!(
            "  {}. {} {}",
            i + 1,
            path.display(),
            if exists { "(exists)" } else { "(not found)" }
        );
    }

    println!("\nEnvironment variables:");
    println!(
        "  MD_TASK_DEV: {}",
        std::env::var("MD_TASK_DEV").unwrap_or_else(|_| "not set".to_string())
    );

    if let Some(project_root) = find_project_root() {
        println!("  Project root: {}", project_root.display());
    } else {
        println!("  Project root: not detected");
    }
}

#[allow(dead_code)]
pub fn show_config_status() {
    let search_paths = get_config_search_paths();

    println!("=== md-task Configuration Status ===");
    println!("Configuration file search order:");
    for (i, path) in search_paths.iter().enumerate() {
        let exists = path.exists();
        let status = if exists {
            "✅ exists"
        } else {
            "❌ not found"
        };
        println!("  {}. {} {}", i + 1, path.display(), status);
    }

    println!("\nEnvironment:");
    println!(
        "  MD_TASK_DEV: {}",
        std::env::var("MD_TASK_DEV").unwrap_or_else(|_| "not set".to_string())
    );

    if let Some(project_root) = find_project_root() {
        println!("  Project root: {}", project_root.display());
    } else {
        println!("  Project root: not detected");
    }

    // 現在の有効な設定を表示
    let config = load_config();
    println!("\nCurrent active configuration:");
    println!("  Task file: {}", config.file_paths.task_file);
    println!(
        "  Default priority: {}",
        config.task_management.default_priority
    );
    println!("  Auto format: {}", config.task_management.auto_format);
}
