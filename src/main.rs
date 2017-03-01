use std::io;
use std::path::Path;

fn get_files_in_dir(dir_path: &Path) -> io::Result<()> {
    for entry in std::fs::read_dir(dir_path)? {
        let path = entry?.path();
        if path.is_dir() {
            get_files_in_dir(&path);
        }
        println!("{}", path.display());
    }
    Ok(())
}


fn main() {
    let test_dir = Path::new(".");
    get_files_in_dir(test_dir).expect(&format!("Can't get files from directory \"{}\"", test_dir.display()));
}
