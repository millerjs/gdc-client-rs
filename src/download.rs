//! GDC Download functionality

use ::DEFAULT_HOST;
use ::XAuthToken;
use ::get_token;
use ::read_file;
use clap::ArgMatches;
use hyper::header::Headers;
use sledge;
use std::io::prelude::*;
use std::io;
use std::process::exit;


/// Extract args to start a download session
pub fn download(matches: &ArgMatches)
{
    let urls = construct_urls(matches);

    // Headers
    let mut headers = Headers::new();

    // Token
    if let Some(token) = get_token(matches) {
        debug!("Read token of {} bytes", token.len());
        headers.set(XAuthToken(token))
    }

    // Target
    let target = match matches.is_present("STDOUT") {
        true => sledge::download::DownloadTarget::StdOut,
        false => sledge::download::DownloadTarget::Default,
    };

    // Start download
    debug!("Downloading {:?}", urls);
    info!("Downloading {} {}.", urls.len(), match urls.len() {1 => "id", _ => "ids"});
    let result = sledge::download::download_urls(urls, target, headers);

    match result {
        Ok(ok) => info!("{}\n", ok),
        Err(err) => error!("{}\n", err),
    }
}


/// Reads in a file with an expected GDC manifest format
pub fn load_ids_from_manifest(manifest_path: &str) -> Result<Vec<String>, io::Error>
{
    let mut ids = vec![];
    let file = try!(read_file(manifest_path));

    for (i, line) in file.split("\n").collect::<Vec<_>>()[1..].iter().enumerate() {
        match line.split("\t").nth(0) {
            Some(id) => ids.push(id.to_string()),
            None => error_and_exit!("Poorly formated manifest (line {}): {}", i, line),
        }
    }

    Ok(ids)
}


/// Read DIDs from manifest and cli args and construct a url for each
pub fn construct_urls(matches: &ArgMatches) -> Vec<String>
{
    let host = matches.value_of("HOST").unwrap_or(DEFAULT_HOST);
    let mut dids = vec![];

    if let Some(ids) = matches.values_of("UUIDS") {
        dids.extend(ids.map(|s| s.to_string()).collect::<Vec<_>>());
        debug!("Loaded {:?} command line.", dids);
    } else {
        debug!("No ids specified on the command line.")
    }

    if let Some(path) = matches.value_of("MANIFEST") {
        match load_ids_from_manifest(path) {
            Err(e) => error_and_exit!("Unable to read manifest '{}': {}", path, e),
            Ok(manifest_ids) => {
                debug!("Loaded {:?} from manifest: {}.", manifest_ids, path);
                dids.extend(manifest_ids)
            },
        }
    } else {
        debug!("No manifest specified.")
    }

    if dids.len() == 0 {
        error_and_exit!("No ids to download.");
    }

    dids.iter().map(|did| format!("{}/data/{}", host, did)).collect()
}
