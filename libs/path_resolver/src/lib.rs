use std::{
    env,
    path::{Path, PathBuf},
};

pub fn resolve_path<S: AsRef<Path>>(path: S) -> PathBuf {
    let path = path.as_ref().to_str().unwrap();

    if path.starts_with("~") {
        let home_path = env::var("HOME").expect("Could not find $HOME");
        let mut pathbuf = PathBuf::from(home_path);
        if path.starts_with("~/") {
            pathbuf = pathbuf.join(&path[2..])
        }
        pathbuf
    } else {
        PathBuf::from(path)
    }
}
