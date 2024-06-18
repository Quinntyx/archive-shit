pub mod compress;
mod fs;
pub mod schema;
pub use compress::CompressedArchive;
pub use schema::CompressionSchema;
use serde::{Deserialize, Serialize};

use fs::*;
use std::fs::{create_dir_all, File, FileTimes};
use std::io::{BufReader, Read as _, Write as _};
use std::path::PathBuf;

#[derive(Clone)]
pub struct SerializedArchive(pub Vec<u8>, bool);

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Archive(pub Vec<ArchiveEntry>, bool);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArchiveEntry {
    name: String,
    content: Vec<u8>,
    meta: FileMetadata,
}

impl SerializedArchive {
    pub fn compress(self, schema: CompressionSchema) -> CompressedArchive {
        if self.1 {
            println!("Converting serialized archive to compressed archive.")
        }
        CompressedArchive::new(self, schema)
    }

    pub fn deserialize(self) -> Archive {
        if self.1 {
            println!("Deserializing archive.")
        }
        Archive(
            bincode::deserialize(&self.0).expect("Archive can be deserialized"),
            self.1,
        )
    }
}

impl Archive {
    pub fn serialize(self) -> SerializedArchive {
        if self.1 {
            println!("Serializing archive.")
        }
        SerializedArchive(
            bincode::serialize(&self).expect("Archive can be serialized"),
            self.1,
        )
    }

    pub fn new(files: &[PathBuf], debug: bool) -> Self {
        let files: Vec<PathBuf> = files.iter().map(|i| recurse(i.into())).flatten().collect();
        let filecount = files.len();
        let mut archive = Archive(vec![], debug);
        for (n, file) in files.iter().enumerate() {
            let entry = ArchiveEntry::new(file.clone().into(), archive.1);
            if archive.1 {
                println!("Archiving {} [{}/{}]", entry.name.clone(), n + 1, filecount)
            }
            archive.0.push(entry);
        }
        archive
    }

    pub fn write_entries_to_disk(&self) {
        let entrycount = self.0.len();
        for (n, entry) in self.0.iter().enumerate() {
            if self.1 {
                println!("Writing {} [{}/{}]", entry.name, n + 1, entrycount)
            }
            let path: PathBuf = entry.name.clone().into();
            create_dir_all(path.parent().expect("Path is not root or prefix"))
                .expect("Directories should be creatable");

            let mut file = File::create(path).expect("File should be creatable");
            file.write(&entry.content).expect("File should be writable");

            let mut times = FileTimes::new();
            entry.meta.accessed.map(|i| times.set_accessed(i));
            entry.meta.modified.map(|i| times.set_modified(i));

            #[cfg(windows)] {
                use std::os::windows::fs::FileTimesExt;
                entry.meta.created.map(|i| times.set_created(i));
            }

            let mut perms = file.metadata().expect("Should be able to access file metadata").permissions();

            entry.meta.permissions.update_std(&mut perms);

            file.set_times(times).expect("Should be able to set times");
            file.set_permissions(perms).expect("Should be able to set permissions");
        }
    }
}

impl ArchiveEntry {
    pub fn new(file: PathBuf, debug: bool) -> Self {
        let filename = file.clone().into_os_string().into_string().unwrap();
        if debug {
            println!("Reading {}", filename.clone())
        }

        let file = File::open(&file).expect("File should exist");
        let meta = FileMetadata::new(file.metadata().expect("File should provide metadata"));

        let f = BufReader::new(file);
        ArchiveEntry {
            name: filename,
            content: f
                .bytes()
                .map(|i| i.expect("Bytes should be valid"))
                .collect(),
            meta,
        }
    }
}
