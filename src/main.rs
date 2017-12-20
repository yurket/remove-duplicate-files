use std::path::Path;
use std::fs;
use std::collections::HashSet;

extern crate sha2;
use sha2::{Sha256, Digest};

fn remove_file_duplicates(dir_path: &Path) -> Result<u32, std::io::Error>{
    let mut n_duplicates: u32 = 0;
    let mut file_hashes = HashSet::new();

    for entry in std::fs::read_dir(dir_path)? {
        let path_buf = entry?.path();
        if path_buf.is_dir() {
            continue;
        }
        let mut file = fs::File::open(path_buf.as_path())?;
        let hash = Sha256::digest_reader(&mut file)?;

        if file_hashes.contains(& hash) {
            std::fs::remove_file(path_buf.as_path())?;
            n_duplicates += 1;
        }
        else {
            file_hashes.insert(hash);
        }
        println!("{:x}\t{:?}", hash, path_buf.as_path());
    }

    Ok(n_duplicates)
}


fn main() {
    let test_dir = Path::new(".");

    match remove_file_duplicates(test_dir) {
        Ok(n_duplicates) => println!("removed {} duplicates", n_duplicates),
        Err(e) => println!("Error: {}", e),
    }
}
