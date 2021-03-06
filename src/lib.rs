use std::io::Cursor;
mod cache_requirements;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use sha256::digest_bytes;
use tar::Archive;

struct Storage {
    #[allow(dead_code)]
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    #[allow(dead_code)]
    location_supported: bool,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub command: CommandType,
    pub bucket_id: Option<String>,
}

#[derive(Clone, Debug)]
pub enum CommandType {
    Upload,
    Download,
}

fn generate_archive(requirements: Vec<String>) -> Vec<u8> {
    let mut tar_gz: Vec<u8> = vec![];

    {
        let mut tar = tar::Builder::new(&mut tar_gz);

        for requirement in requirements.iter() {
            tar.append_dir_all(requirement, requirement).unwrap();
        }

        tar.finish().unwrap();
    }

    let mut compressed_tar: Vec<u8> = vec![];

    zstd::stream::copy_encode(Cursor::new(tar_gz), &mut compressed_tar, 10).unwrap();

    println!("🚀 Build archive generated");

    compressed_tar
}

async fn upload_build_cache(bucket: Bucket) {
    let requirements = cache_requirements::get_cache_requirements("next");

    let archive = generate_archive(requirements);

    let checksum = hex::encode(digest_bytes(&archive));

    // todo: delete the old build cache, and update cache url
    bucket
        .put_object(format!("/{checksum}"), &archive)
        .await
        .unwrap();

    println!("☁️  Uploaded build cache to S3");
}

async fn download_build_cache(bucket: Bucket, id: &str) {
    let (data, _) = bucket.get_object(id).await.unwrap();

    // decompress data
    let mut decompressed_data: Vec<u8> = vec![];

    zstd::stream::copy_decode(Cursor::new(data), &mut decompressed_data).unwrap();

    // extract the tar with the decompressed data
    let mut archive = Archive::new(Cursor::new(decompressed_data));

    archive.unpack(".").unwrap();
}

fn get_bucket() -> Bucket {
    let vultr = Storage {
        name: "hyperpack".into(),
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
        bucket: "builds".into(),
        location_supported: false,
    };

    Bucket::new(&vultr.bucket, vultr.region, vultr.credentials)
        .unwrap()
        .with_path_style()
}

pub async fn run(options: Options) {
    let bucket = get_bucket();

    match options.command {
        CommandType::Upload => upload_build_cache(bucket).await,
        CommandType::Download => download_build_cache(bucket, &options.bucket_id.unwrap()).await,
    }
}
