#![crate_name = "gdcclient"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate hyper;
extern crate clap;
extern crate env_logger;
extern crate sledge;

use clap::ArgMatches;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

#[macro_use]
pub mod macros;
pub mod download;

/// Default GDC API hostname and scheme
pub static DEFAULT_HOST: &'static str = "https://gdc-api.nci.nih.gov";

/// Custom GDC Headers
header! { (XAuthToken, "X-Auth-Token") => [String] }

/// Try to read a token given cli arg matches
pub fn get_token(matches: &ArgMatches) -> Option<String>
{
    if let Some(t) = matches.value_of("TOKEN") {
        return Some(t.to_owned())
    } else if let Some(token_path) = matches.value_of("TOKEN_FILE") {
        match read_file(token_path) {
            Ok(token) => return Some(token),
            Err(e) => error_and_exit!("Unable to read token file {}: {}", token_path, e),
        }
    }
    None
}

/// Setup logging (cli arg overwrites env var for dtt crate)
pub fn setup_logging(matches: &ArgMatches)
{
    let rust_log = env::var("RUST_LOG").unwrap_or("".to_owned());
    let log_level = match matches.occurrences_of("v") {
        0 => "sledge=info,gdcclient=info",
        _ => "sledge=debug,gdcclient=debug",
    };

    env::set_var("RUST_LOG", &*format!("{},{}", rust_log, log_level));
    env_logger::init().unwrap();

    debug!("Set log level to {}", log_level);
}

/// Try to read a file given a str path
pub fn read_file(path: &str) -> Result<String, io::Error>
{
    let mut file = try!(File::open(Path::new(path)));
    let mut buffer = String::new();
    let bytes = try!(file.read_to_string(&mut buffer));
    debug!("Read {} bytes from {}", bytes, path);
    Ok(buffer)
}
