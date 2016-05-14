use std::fs;
use std::path::Path;


/// Returns size in bytes of a file or directory. For directories, it recursively gets all of the
/// sizes of its children.
pub fn get_size<P: AsRef<Path>>(path: P) -> std::io::Result<u64> {
    let metadata = try!(fs::metadata(&path));
    let mut size = metadata.len();
    if metadata.is_dir() {
        for entry in try!(fs::read_dir(&path)) {
            size += try!(get_size(try!(entry).path()));
        }
    }
    Ok(size)
}
