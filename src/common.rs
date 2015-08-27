#![macro_export]

use std::fs::File;
use std::io::Read;
use std::result::Result;
use std::error::Error;
use std::path::Path;
use std::convert::From;
use std::io;
use rumblebars::Template;
use rumblebars::EvalContext;
use rumblebars;
use rustc_serialize::json;
#[derive(Debug)]
pub enum SiteGenError {
//    RumblebarsParse(rumblebars::parse::ParseError),
    Io(io::Error),
    Other(String),
    JsonEncode(json::EncoderError),
    JsonParse(json::ParserError),
}

impl From<io::Error> for SiteGenError {
    fn from(e: io::Error) -> SiteGenError {
        SiteGenError::Io(e)
    }
}


/*impl From<(rumblebars::parse::ParseError, Option<String>)> for SiteGenError {
    fn from(e: (rumblebars::parse::ParseError, Option<String>)) -> SiteGenError {
        SiteGenError::RumblebarsParse(e.0)
    }
}*/

impl From<json::ParserError> for SiteGenError {
    fn from(e: json::ParserError) -> SiteGenError {
        SiteGenError::JsonParse(e)
    }
}

impl From<json::EncoderError> for SiteGenError {
    fn from(e: json::EncoderError) -> SiteGenError {
        SiteGenError::JsonEncode(e)
    }
}

/*impl From<rumblebars::parse::ParseError> for SiteGenError {
    fn from(e: rumblebars::parse::ParseError) -> SiteGenError {
        SiteGenError::RumblebarsParse(e)
    }
}*/

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

pub fn create_template(layout_path: &Path) -> SiteGenResult<Template> {
    let path_to_string = try!(read_file_to_string(layout_path));
    let template = rumblebars::parse(&path_to_string).unwrap();
    Ok(template)
}

pub fn register_partial(partial_path: &Path, context: &mut EvalContext) -> SiteGenResult<()> {
    let partial_name = try!(path_filename(partial_path));
    let partial_template = try!(create_template(partial_path));
    let _ = context.register_partial(String::from(partial_name), partial_template);
    Ok(())
}

pub fn read_file_to_string(path: &Path) -> SiteGenResult<String> {
    let mut file = try!(File::open(path));
    let mut string_from_file = String::new();
    try!(file.read_to_string(&mut string_from_file));
    return Ok(string_from_file);
}

pub fn path_filename(path: &Path) -> SiteGenResult<String> {
    let os_str_filename = try_option_empty_error!(path.file_stem(), String::from("file name empty"));
    let str_filename: &str = try_option_empty_error!(os_str_filename.to_str(),
        String::from("Couldn't convert os_str to str"));
    return Ok(String::from(str_filename));
}
