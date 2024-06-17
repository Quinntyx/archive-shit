pub mod compress;
mod fs;
pub use compress::CompressedArchive;
use serde::{Serialize, Deserialize};

use std::path::PathBuf;
use std::fs::{File, create_dir_all, write};
use std::io::{BufReader, Read as _};
use fs::*;

pub struct SerializedArchive(pub Vec<u8>, bool);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Archive(pub Vec<ArchiveEntry>, bool);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ArchiveEntry {
    name: String,
    content: Vec<u8>,
}

impl SerializedArchive {
    pub fn compress(self) -> CompressedArchive {
        if self.1 { println!("Converting serialized archive to compressed archive.") }
        CompressedArchive::new(self)
    }

    pub fn deserialize(self) -> Archive {
        if self.1 { println!("Deserializing archive.") }
        Archive(bincode::deserialize(&self.0).expect("Archive can be deserialized"), self.1)
    }
}

impl Archive {
    pub fn serialize(self) -> SerializedArchive {
        if self.1 { println!("Serializing archive.") }
        SerializedArchive(bincode::serialize(&self).expect("Archive can be serialized"), self.1)
    }

    pub fn new(files: &[PathBuf], debug: bool) -> Self {
        let files: Vec<PathBuf> = files.iter().map(|i| recurse(i.into())).flatten().collect();
        let filecount = files.len();
        let mut archive = Archive(vec!(), debug);
        for (n, file) in files.iter().enumerate() {
            let entry = ArchiveEntry::new(file.clone().into(), archive.1);
            if archive.1 { println!("Archiving {} [{}/{}]", entry.name.clone(), n + 1, filecount) }
            archive.0.push(entry);
        }
        archive
    }

    pub fn write_entries_to_disk(&self) {
        let entrycount = self.0.len();
        for (n, entry) in self.0.iter().enumerate() {
            if self.1 { println!("Writing {} [{}/{}]", entry.name, n + 1, entrycount) }
            let path: PathBuf = entry.name.clone().into();
            create_dir_all(path.parent().expect("Path is not root or prefix")).expect("Directories should be creatable");
            write(path, entry.content.clone()).expect("File should be writable");
        }
    }
}

impl ArchiveEntry {
    pub fn new(file: PathBuf, debug: bool) -> Self {
        let filename = file.clone().into_os_string().into_string().unwrap();
        if debug { println!("Reading {}", filename.clone()) }

        let f = BufReader::new(File::open(&file).expect("File should exist"));
        ArchiveEntry {
            name: filename,
            content: f.bytes()
                .map(|i| i.expect("Bytes should be valid"))
                .collect(),
        }
    }
}
