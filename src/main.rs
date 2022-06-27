use std::{
    fs::File,
    io::{Cursor, Read, Write},
};
mod cache_requirements;
mod helpers;

pub fn generate_archive(requirements: Vec<String>) {
    let tar_gz = File::create("archive.tar.gz").unwrap();
    let mut tar = tar::Builder::new(tar_gz);

    for requirement in requirements {
        tar.append_dir_all(".", requirement).unwrap();
    }

    tar.finish().unwrap();

    // let mut buf: Vec<u8> = vec![];

    // tar.into_inner().unwrap().read_to_end(&mut buf).unwrap();

    // let compressed_tar: Vec<u8> = vec![];

    // zstd::stream::copy_encode(Cursor::new(buf), compressed_tar.clone(), 10).unwrap();

    // let mut file = File::create("archive.zst").unwrap();

    // file.write(&compressed_tar).unwrap();
}

fn main() {
    let requirements = cache_requirements::get_cache_requirements("next");

    generate_archive(requirements);
}
