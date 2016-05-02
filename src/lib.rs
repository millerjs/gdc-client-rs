#![crate_name = "gdcclient"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate hyper;
extern crate clap;
extern crate env_logger;
extern crate sledge;

use clap::ArgMatches;
use sledge::errors::DownloadError;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;
use hyper::header::Headers;


pub static DEFAULT_HOST: &'static str = "https://gdc-api.nci.nih.gov";


header! { (XAuthToken, "X-Auth-Token") => [String] }


/// Extract args to start a download session
pub fn download(matches: &ArgMatches)
{
    let host = &*matches.value_of("HOST").unwrap_or(DEFAULT_HOST);
    let token = matches.value_of("TOKEN");

    let mut dids = match matches.values_of("UUIDS") {
        Some(ids) => ids.map(|s| s.to_string()).collect(),
        None => vec![],
    };

    if let Some(path) = matches.value_of("MANIFEST") {
        match load_ids_from_download_manifest(path) {
            Ok(manifest_ids) => dids.extend(manifest_ids),
            Err(e) => {
                error!("Unable to read manifest '{}': {}", path, e);
                exit(1);
            }
        }
    }

    if dids.len() == 0 {
        println!("No ids to download.");
        exit(1)
    }

    let urls = dids.iter().map(|did| format!("{}/data/{}", host, did)).collect();

    let mut headers = Headers::new();
    if let Some(t) = token {
        headers.set(XAuthToken(t.to_owned()));
    };

    match sledge::download::download_urls(urls, headers) {
        Ok(ok) => info!("{}\n", ok),
        Err(err) => error!("{}\n", err),
    }
}


/// Setup logging (cli arg overwrites env var for dtt crate)
pub fn setup_logging(matches: &ArgMatches)
{
    let rust_log = env::var("RUST_LOG").unwrap_or("".to_owned());
    let log_level = match matches.occurrences_of("v") {
        0 => "sledge=info,gdcclient=info",
        _ => "sledge=debug,gdcclient=debug"
    };

    env::set_var("RUST_LOG", &*format!("{},{}", rust_log, log_level));
    env_logger::init().unwrap();
}


pub fn load_ids_from_download_manifest(manifest_path: &str) -> Result<Vec<String>, DownloadError>
{
    let path = Path::new(manifest_path);
    let file = try!(File::open(path));
    let reader = BufReader::new(&file);

    let mut ids = vec![];
    for (i, line) in reader.lines().enumerate() {
        if i == 0 { continue }
        let l = try!(line).to_owned();
        match l.split("\t").nth(0) {
            Some(id) => ids.push(id.to_string()),
            None => {
                error!("Poorly formated manifest (line {}): {}", i, l);
                exit(1)
            }
        }
    }
    Ok(ids)
}
