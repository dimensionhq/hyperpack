use std::io;

// This function use the convenient `copy_encode` method
fn zstd_compress(level: i32) {
    zstd::stream::copy_encode(io::stdin(), io::stdout(), level).unwrap();
}
