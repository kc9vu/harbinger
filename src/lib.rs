use std::path::{Path, PathBuf};
use std::sync::LazyLock;

#[cfg(target_os = "windows")]
static DIRS: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
    let mut v = vec![];
    if let Ok(exe) = std::env::current_exe() && let Some(pwd) = exe.parent() {
        v.push(pwd.to_path_buf());
    }
    if let Some(home) = std::env::home_dir() {
        v.push(home.join(".config"))
    }
    v
});
// #[cfg(target_os = "windows")]
#[cfg(any(target_os = "macos", target_os = "linux"))]
static DIRS: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
    let mut v = vec![];
    if let Ok(exe) = std::env::current_exe() && let Some(pwd) = exe.parent() {
        v.push(pwd.to_path_buf());
    }
    if let Some(home) = std::env::home_dir() {
        v.push(home.join(".config"))
    }
    v.push(Path::new("/etc").to_path_buf());
    v
});

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("extern crate $1: $2")]
    Extern(&'static str, String),
}

/// 读取环境变量, 并判断文件存在
pub fn env(key: &'static str) -> Option<PathBuf> {
    std::env::var_os(key)
        .as_deref()
        .map(Path::new)
        .filter(|p| p.exists())
        .map(Path::to_path_buf)
}

/// 从常见的配置目录中搜索
pub fn find(name: &str) -> Option<PathBuf> {
    for dir in DIRS.iter() {
        if name.contains('/') {
            let path = dir.join(name);
            if path.exists() {
                return Some(path);
            }
        } else if name.contains('.') {
            let path = dir.join(name);
            if path.exists() {
                return Some(path);
            }
        } else {
            let config_dir = dir.join(name);
            let path = config_dir.join("config.toml");
            if path.exists() {
                return Some(path);
            }

            let path = config_dir.join("config.json");
            if path.exists() {
                return Some(path);
            }
        }
    }

    None
}
