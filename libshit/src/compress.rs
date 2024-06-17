use crate::SerializedArchive;

use std::path::PathBuf;
use std::fs::{write, File};
use std::io::{BufReader, Read};
use std::io;

pub struct CompressedArchive(pub Vec<u8>, bool);

impl CompressedArchive {
    pub fn new(archive: SerializedArchive) -> Self {
        if archive.1 { println!("Compressing archive.") }
        let mut comp = flate3::Compressor {
            options: flate3::Options {
                dynamic_block_size: true, 
                block_size: 0x2000, 
                matching: true,
                probe_max: 100, 
                lazy_match: true,
                match_channel_size: 1000,
            }
        };
        let new_archive = Self(comp.deflate(&archive.0), archive.1);
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

        let content = f.bytes()
            .map(|i| i.expect("Bytes should be valid"))
            .collect();
        
        CompressedArchive(content, debug)
    }

    pub fn decompress(self) -> SerializedArchive {
        if self.1 { println!("Decompressing archive.") }
        SerializedArchive(flate3::inflate(&self.0), self.1)
    }

    pub fn write(&self, target: PathBuf) -> io::Result<()> {
        if self.1 { println!("Writing archive to disk.") }
        write(target, &self.0)
    }
}
