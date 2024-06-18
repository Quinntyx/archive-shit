fn main() {
    let target = std::env::args()
        .last()
        .expect("Archive name should be specified");
    libshit::CompressedArchive::new_from_disk(target.into(), true)
        .decompress()
        .deserialize()
        .write_entries_to_disk();
}
