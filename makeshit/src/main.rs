use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let archive_files: Vec<PathBuf> = args[1..args.len() - 1]
        .iter()
        .map(|i| i.into())
        .collect();
    let archive_name = args.last()
        .expect("Archive name should be specified");

    libshit::Archive::new(&archive_files, true)
        .serialize()
        .compress()
        .write(archive_name.into())
        .expect("Archive should be writable");
}
