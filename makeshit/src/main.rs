use std::path::PathBuf;
use libshit::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let archive_files: Vec<PathBuf> = args[1..args.len() - 1]
        .iter()
        .map(|i| i.into())
        .collect();
    let archive_name = args.last()
        .expect("Archive name should be specified");

    let base_archive = Archive::new(&archive_files, true)
        .serialize();

    let mut best_archive: (usize, Option<CompressedArchive>) = (0, None);

    println!("Beginning compression phase. Testing all schemas to find optimal ratio.");
    for schema in CompressionSchema::get_all_schemas() {
        let new_archive = base_archive.clone().compress(schema);
        if best_archive.1.is_none() || best_archive.0 > new_archive.0.len() {
            best_archive = (new_archive.0.len(), Some(new_archive));
            println!("New current best: schema[{}]", schema.schema_string());
        }
    }

    best_archive.1.unwrap().write(archive_name.into()).expect("Archive should be writable");
}
