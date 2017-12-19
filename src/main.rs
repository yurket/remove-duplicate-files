use std::path::{Path, PathBuf};

fn get_files_in_dir_nonrecursively(dir_path: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = vec![];

    for entry in std::fs::read_dir(dir_path).expect("read_dir call failed") {
        paths.push(entry.unwrap().path());
    }
    paths
}

fn main() {
    let test_dir = Path::new(".");

    print!("{:?}", get_files_in_dir_nonrecursively(test_dir));
}
