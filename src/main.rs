use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{copy, BufReader};
use std::time::Instant;

// TODO: Add suport brotli compression later, user crate brotli
fn main() {
    let mut args = args().skip(1);
    let source_path = args.next().expect("Usage: `source` `target`");
    let target_path = args.next().expect("Usage: `source` `target`");

    let source_file = OpenOptions::new()
        .read(true)
        .open(&source_path)
        .expect("Failed to open the file.");
    let target_file = File::create(&target_path).expect("Failed to create target path");

    let mut input = BufReader::new(source_file);
    let mut encoder = GzEncoder::new(target_file, Compression::default());

    let start = Instant::now();

    copy(&mut input, &mut encoder).expect("Failed to compress data");

    let output = encoder.finish().expect("Failed to compress data");

    let source_len = input
        .get_ref()
        .metadata()
        .expect("Failed to retrieve source metadata")
        .len();
    let target_len = output
        .metadata()
        .expect("Failed to retrieve target metadata")
        .len();

    println!("Source len: {:?}", source_len);
    println!("Target len: {:?}", target_len);
    println!("Time: {:?}", start.elapsed());
}
