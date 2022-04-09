use std::{env, path::PathBuf};

pub fn get_home_path() -> Option<PathBuf> {
    if let Ok(home_path) = env::var("HOME") {
        let path = PathBuf::from(home_path);
        if path.exists() {
            return Some(PathBuf::from(path));
        }
    }

    if let Ok(user) = env::var("USER") {
        let path = PathBuf::from(format!("/home/{user}"));
        if path.exists() {
            return Some(path);
        }
    }

    return None;
}
