use crate::schema::CompressionSchema;
use crate::SerializedArchive;

use std::fs::{write, File};
use std::io;
use std::io::{BufReader, Read};
use std::path::PathBuf;

#[derive(Clone)]
pub struct CompressedArchive(pub Vec<u8>, bool);

impl CompressedArchive {
    pub fn new(archive: SerializedArchive, schema: CompressionSchema) -> Self {
        if archive.1 {
            println!(
                "Compressing archive with schema[{}]",
                schema.schema_string()
            )
        }

        let mut new_archive = Self(schema.compress(&archive.0), archive.1);
        new_archive.0.insert(0, schema as u8); // TODO: figure out how to do this better
        let ratio = new_archive.0.len() as f64 / archive.0.len() as f64 * 100f64;
        if archive.1 {
            println!(
                "Compressed: {} -> {} ({:2}%)",
                archive.0.len(),
                new_archive.0.len(),
                ratio
            )
        }
        new_archive
    }

    pub fn new_from_disk(file: PathBuf, debug: bool) -> Self {
        if debug {
            let filename = file
                .clone()
                .into_os_string()
                .into_string()
                .expect("Path should be valid.");
            println!("Reading {} as compressed archive.", filename)
        }

        let f = BufReader::new(File::open(&file).expect("File should exist"));

        let content = f
            .bytes()
            .map(|i| i.expect("Bytes should be valid"))
            .collect();

        CompressedArchive(content, debug)
    }

    pub fn get_schema(&self) -> CompressionSchema {
        CompressionSchema::from_id(self.0.first().expect("Should have initial byte").clone())
    }

    pub fn decompress(self) -> SerializedArchive {
        let schema = self.get_schema();
        if self.1 {
            println!(
                "Detected archive compression schema [{}].",
                schema.schema_string(),
            )
        }

        if self.1 {
            println!("Decompressing archive.")
        }

        SerializedArchive(schema.decompress(&self.0[1..]), self.1)
    }

    pub fn write(&self, target: PathBuf) -> io::Result<()> {
        if self.1 {
            println!("Writing archive to disk.")
        }
        write(target, &self.0)
    }
}
