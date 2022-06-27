use std::{
    fs::File,
    io::{Cursor, Write},
};
mod cache_requirements;
mod helpers;

pub fn generate_archive(requirements: Vec<String>) {
    let tar_gz = File::create("archive.tar.gz").unwrap();
    let mut tar = tar::Builder::new(tar_gz);

    for requirement in requirements {
        println!("{requirement}");
        tar.append_dir_all(".", requirement).unwrap();
    }

    tar.finish().unwrap();

    let buf: Vec<u8> = std::fs::read("archive.tar.gz").unwrap();

    let mut compressed_tar: Vec<u8> = vec![];

    zstd::stream::copy_encode(Cursor::new(buf), &mut compressed_tar, 10).unwrap();

    let mut file = File::create("archive.zst").unwrap();

    file.write(&compressed_tar).unwrap();
}

fn main() {
    let start = std::time::Instant::now();

    let requirements = cache_requirements::get_cache_requirements("next");

    generate_archive(requirements);

    println!(
        "Execution completed in {} seconds",
        start.elapsed().as_secs_f32()
    );
}
