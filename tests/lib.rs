extern crate tempdir;
extern crate rand;
extern crate du;

use std::fs::File;
use std::path::PathBuf;

use rand::Rng;


fn create_file_of_size_in(size: u64, dir: &tempdir::TempDir) -> std::io::Result<PathBuf> {
    let mut rng = rand::thread_rng();
    let filename: String = rng.gen_ascii_chars().take(20).collect();
    let path = dir.path().join(filename);

    let f = try!(File::create(&path));
    try!(f.set_len(size));

    Ok(path)
}


#[test]
fn get_size_of_file() {
    let dir = tempdir::TempDir::new("test").unwrap();
    let path = create_file_of_size_in(20, &dir).unwrap();

    assert_eq!(du::get_size(path.as_path()).unwrap(), 20);
}


#[test]
fn get_size_of_file_with_string_for_path() {
    let dir = tempdir::TempDir::new("test").unwrap();
    let path = create_file_of_size_in(20, &dir).unwrap();

    let path_str = &path.to_str().unwrap();

    assert_eq!(du::get_size(&path_str).unwrap(), 20);
}


#[test]
fn get_size_of_dir() {
    let dir = tempdir::TempDir::new("test").unwrap();

    assert_eq!(du::get_size(dir.path()).unwrap(), 40);
    let _ = create_file_of_size_in(20, &dir);

    assert_eq!(du::get_size(dir.path()).unwrap(), 80);
}
