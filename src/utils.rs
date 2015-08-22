use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

pub fn read_file_to_string(path: &Path) -> Result<String> {
    let mut file = try!(File::open(path));
    let mut string_from_file = String::new();
    try!(file.read_to_string(&mut string_from_file));
    return Ok(string_from_file);
}

pub fn path_filename(path: &Path) -> Result<String> {
    let os_str_filename = match path.file_name() {
        Some(os_str) => os_str,
        None => return Err(Error::new(ErrorKind::Other, "file name empty")),
    };
    let str_filename: &str = match os_str_filename.to_str() {
        Some(filename) => filename,
        None => return Err(Error::new(ErrorKind::Other, "Couldn't convert os_str to str")),
    };
    return Ok(String::from(str_filename));
}
