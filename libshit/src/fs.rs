use std::fs::read_dir;
use std::path::PathBuf;

pub fn recurse(path: PathBuf) -> Vec<PathBuf> {
    let Ok(entries) = read_dir(path.clone()) else {
        return vec![path];
    };
    entries
        .flatten()
        .flat_map(|entry| {
            let Ok(meta) = entry.metadata() else {
                return vec![];
            };
            if meta.is_dir() {
                return recurse(entry.path());
            }
            if meta.is_file() {
                return vec![entry.path()];
            }
            vec![]
        })
        .collect()
}
