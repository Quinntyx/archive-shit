use serde::{Deserialize, Serialize};
use std::fs::read_dir;
use std::fs::{Metadata, Permissions};
use std::path::PathBuf;
use std::time::SystemTime;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileMetadata {
    pub permissions: FilePermissions,
    pub accessed: Option<SystemTime>,
    pub modified: Option<SystemTime>,
    pub created: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FilePermissions {
    pub mode: Option<u32>,
    pub readonly: bool, // used only on Windows platforms.
}

impl FileMetadata {
    pub fn new(meta: Metadata) -> Self {
        Self {
            permissions: FilePermissions::new(meta.permissions()),
            accessed: meta.accessed().ok(),
            modified: meta.modified().ok(),
            created: meta.created().ok(),
        }
    }
}

impl FilePermissions {
    pub fn new(perms: Permissions) -> Self {
        Self {
            mode: Self::mode(&perms),
            readonly: perms.readonly(),
        }
    }

    fn mode(perms: &Permissions) -> Option<u32> {
        #[cfg(windows)] {
            None
        } 
        #[cfg(unix)] {
            use std::os::unix::fs::PermissionsExt as _;
            Some(perms.mode())
        }
    }

    pub fn update_std(&self, perms: &mut Permissions) {
        #[cfg(windows)] {
            perms.set_readonly(self.readonly);
        }
        #[cfg(unix)] {
            use std::os::unix::fs::PermissionsExt as _;
            self.mode.map(|i| perms.set_mode(i));
        }
    }
}
