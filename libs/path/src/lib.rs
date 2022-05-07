use std::{
    env,
    fmt::{Debug, Display},
    fs, io,
    ops::Deref,
    path::PathBuf,
};

pub fn resolve<S: AsRef<Path>>(path: S) -> Result<PathBuf, io::Error> {
    path = path.as_ref();
    let mut pathbuf = if path.starts_with("~") {
        let home_path = env::var("HOME").expect("Could not find $HOME");
        let mut pathbuf = PathBuf::from(home_path);
        if pathbuf.starts_with("~/") {
            pathbuf = pathbuf.join(&path[2..])
        }
        pathbuf
    } else {
        PathBuf::from(path)
    };

    if !pathbuf.exists() {
        return Err(format!("{path} does not exist"));
    }

    pathbuf = pathbuf.canonicalize().unwrap();

    return Ok(Path { path: pathbuf });
}
// #[derive(Debug, Clone)]
// pub struct Path {
//     path: std::path::PathBuf,
// }

// impl Path {
//     pub fn get_home() -> Path {
//         if let Ok(path) = env::var("HOME") {
//             let path = PathBuf::from(path);

//             if !path.exists() {
//                 panic!("$HOME is not a valid path");
//             }

//             return Path { path };
//         }

//         if let Ok(user) = env::var("USER") {
//             let path = PathBuf::from(format!("/home/{user}"));

//             if !path.exists() {
//                 panic!("/home/$USER is not a valid path");
//             }

//             return Path { path };
//         }

//         panic!("No USER or HOME environment variables found")
//     }

//     pub fn resolve(path: &str) -> Result<Path, String> {
//         let mut pathbuf = if path.starts_with("~") {
//             let home_path = env::var("HOME").expect("Could not find $HOME");
//             let mut pathbuf = PathBuf::from(home_path);
//             if pathbuf.starts_with("~/") {
//                 pathbuf = pathbuf.join(&path[2..])
//             }
//             pathbuf
//         } else {
//             PathBuf::from(path)
//         };

//         if !pathbuf.exists() {
//             return Err(format!("{path} does not exist"));
//         }

//         pathbuf = pathbuf.canonicalize().unwrap();

//         return Ok(Path { path: pathbuf });
//     }

//     pub fn filename(&self) -> String {
//         return self.path.file_name().unwrap().to_string_lossy().to_string();
//     }

//     pub fn directory(&self) -> Path {
//         if self.path.is_file() {
//             let path = self
//                 .path
//                 .parent()
//                 .unwrap_or(std::path::Path::new("/"))
//                 .to_path_buf();
//             return Path { path };
//         }

//         return self.clone();
//     }
// }

// impl Display for Path {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.path.to_string_lossy())
//     }
// }

// // impl AsRef<std::path::Path> for Path {
// //     fn as_ref(&self) -> &std::path::Path {
// //         return self.path.as_ref();
// //     }
// // }

// // impl Deref for Path {
// //     type Target = std::path::Path;

// //     fn deref(&self) -> &Self::Target {
// //         self.path.deref()
// //     }
// // }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     pub fn resolves() {
//         let home = env::var("HOME").unwrap();

//         assert_eq!(Path::resolve("~").unwrap().to_string(), home)
//     }
// }
