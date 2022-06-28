use std::io::Cursor;
mod cache_requirements;
mod helpers;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

struct Storage {
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    location_supported: bool,
}

pub fn generate_archive(requirements: Vec<String>) {
    let mut tar_gz: Vec<u8> = vec![];

    {
        let mut tar = tar::Builder::new(&mut tar_gz);

        for requirement in requirements {
            tar.append_dir_all(".", requirement).unwrap();
        }

        tar.finish().unwrap();
    }

    let mut compressed_tar: Vec<u8> = vec![];

    zstd::stream::copy_encode(Cursor::new(tar_gz), &mut compressed_tar, 10).unwrap();
}

fn main() {
    let start = std::time::Instant::now();

    let vultr = Storage {
        name: "vultr".into(),
        region: Region::Custom {
            region: "ewr1".into(),
            endpoint: "https://ewr1.vultrobjects.com".into(),
        },
        credentials: Credentials {
            access_key: Some("7WHIDO523IJT4AHIF2WV".into()),
            secret_key: Some("Tk3YYOIhiVMzz8MH3ZnfHKP5aYh0ESIrqvqtlemM".into()),
            security_token: None,
            session_token: None,
        },
        bucket: "hyperpack".into(),
        location_supported: false,
    };

    let bucket = Bucket::new(&vultr.bucket, vultr.region, vultr.credentials)
        .unwrap()
        .with_path_style();

    println!("{}", bucket.url());

    //     for (list, code) in results {
    //     assert_eq!(200, code);
    //     println!("{:?}", list.contents.len());
    // }

    let requirements = cache_requirements::get_cache_requirements("next");

    generate_archive(requirements);

    println!(
        "Execution completed in {} seconds",
        start.elapsed().as_secs_f32()
    );
}
