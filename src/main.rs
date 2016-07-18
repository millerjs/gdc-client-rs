#[macro_use]
extern crate log;
#[macro_use]
extern crate hyper;
extern crate clap;
extern crate env_logger;
extern crate sledge;
#[macro_use]
extern crate gdcclient;

use clap::App;
use clap::Arg;
use clap::SubCommand;
use clap::ArgGroup;
use std::process::exit;

fn main() {
    let matches = App::new("gdc-data-transfer-tool")
        .version("0.1.0")
        .author("Joshua Miller <jsmiller@uchicago.edu>")
        .about("GDC Data Transfer Tool")
        .subcommand(SubCommand::with_name("download")
                    .about("Download files from the GDC")
                    .arg(Arg::with_name("UUIDS")
                         .help("File UUIDs to download")
                         .multiple(true))
                    .arg(Arg::with_name("MANIFEST")
                         .short("m")
                         .long("manifest")
                         .takes_value(true)
                         .help("Path to manifest with file UUIDs to download"))
                    .arg(Arg::with_name("THREADS")
                         .short("n")
                         .long("threads")
                         .takes_value(true)
                         .help("Number of threads to use during download"))
                    .arg(Arg::with_name("STDOUT")
                         .short("s")
                         .long("stdout")
                         .help("Stream to stdout."))
                    .arg(Arg::with_name("HOST")
                         .short("H")
                         .long("host")
                         .help("Host of the API to download from")
                         .takes_value(true))
                    .arg(Arg::with_name("v")
                         .short("v")
                         .multiple(true)
                         .help("Sets the level of verbosity"))
                    .group(ArgGroup::with_name("TOKEN_GROUP")
                         .args(&["TOKEN_FILE", "TOKEN"]))
                    .arg(Arg::with_name("TOKEN")
                         .short("t")
                         .long("token")
                         .help("Auth token")
                         .takes_value(true))
                    .arg(Arg::with_name("TOKEN_FILE")
                         .short("T")
                         .long("token-file")
                         .help("Auth token file")
                         .takes_value(true)))
        .subcommand(SubCommand::with_name("upload")
                    .about("Uploads files to the GDC"))
        .get_matches();

    gdcclient::setup_logging(&matches);

    if let Some(matches) = matches.subcommand_matches("download") {
        gdcclient::download::download(matches);

    } else if let Some(_) = matches.subcommand_matches("upload") {
        error_and_exit!("Upload functionality is not yet implemented");

    } else {
        error_and_exit!("Please specify a subcommand. For more information try --help");
    }
}
