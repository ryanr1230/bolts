#![macro_export]

use std::fs::File;
use std::io::Read;
use std::result::Result;
use std::error::Error;
use std::path::Path;
use std::convert::From;
use handlebars::Handlebars;
use std::io;

#[derive(Debug)]
pub enum SiteGenError {
    Io(io::Error),
    Other(String),
}


impl From<io::Error> for SiteGenError {
    fn from(e: io::Error) -> SiteGenError {
        SiteGenError::Io(e)
    }
}

pub type SiteGenResult<T> = Result<T, SiteGenError>;

#[macro_export]
macro_rules! convert_err {
    ($result:expr) => (
        $result.map_err(|e| SiteGenError::Other(String::from(e.description())))
    )
}

#[macro_export]
macro_rules! try_option_empty_error {
    ($option:expr, $msg:expr) => (
        match $option {
            Some(thing) => thing,
            None => return Err(SiteGenError::Other($msg)),
        }
    )
}

pub fn register_template(layout_path: &Path, processor: &mut Handlebars) -> SiteGenResult<String> {
    let layout_as_string: String = read_file_to_string(layout_path).unwrap();
    let layout_name = try!(path_filename(layout_path));
    let _ = try_option_empty_error!(processor.register_template_string(&layout_name, layout_as_string).ok(),
        format!("{}{}","Couldn't add template ", layout_name));
    return Ok(layout_name);
}

pub fn read_file_to_string(path: &Path) -> SiteGenResult<String> {
    let mut file = try!(File::open(path));
    let mut string_from_file = String::new();
    try!(file.read_to_string(&mut string_from_file));
    return Ok(string_from_file);
}

pub fn path_filename(path: &Path) -> SiteGenResult<String> {
    let os_str_filename = try_option_empty_error!(path.file_name(), String::from("file name empty"));
    let str_filename: &str = try_option_empty_error!(os_str_filename.to_str(),
        String::from("Couldn't convert os_str to str"));
    return Ok(String::from(str_filename));
}
