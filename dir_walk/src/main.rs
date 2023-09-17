use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let path = Path::new("/home");
    print_dir(&path);
}

fn print_dir(path: &Path) {
    if !path.is_dir() {
        return;
    }

    let mut dirs: VecDeque<PathBuf> = VecDeque::new();
    dirs.push_back(path.to_path_buf());
    loop {
        let current = match dirs.pop_back() {
            Some(path) => path,
            None => break,
        };
        println!("DIR: {}", current.display());
        for entry in fs::read_dir(current).unwrap() {
            let entry = entry.unwrap();
            let path_buf = entry.path();
            if path_buf.is_dir() {
                dirs.push_back(entry.path());
            } else {
                let meta = entry.metadata().unwrap();

                let name = match path_buf.file_name() {
                    Some(name) => match name.to_str() {
                        Some(sname) => sname,
                        None => continue,
                    },
                    None => continue,
                };
                println!("     {} {} bytes", name, meta.len());
            }
        }
    }
}
