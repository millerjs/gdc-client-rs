# GDC Transfer Client

A Rust port of the GDC transfer tool functionality, built for
performance, portability, and reliabiltiy.

## Features
Currently implemented are

* **Download functionality** - Download files serially to disk

To be implemented

* **Resumable downloads** - Restart a failed or canceled download
* **Parallel downloads** - Create multiple HTTP(S) streams to boost performance


## Installation


### Running a pre-compiled binary

To run, download the binary [here](https://github.com/millerjs/gdc-client-rs/releases/)
```bash
tar -zxf <downloaded tar>.tar.gz
cd <downloaded tar>
./gdcclient download <file_id_1> <file_id_1> -T ~/gdc_token.txt
```

#### Download help output

```
Download files from the GDC

USAGE:
    gdcclient download [FLAGS] [OPTIONS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -v               Sets the level of verbosity
    -V, --version    Prints version information

OPTIONS:
    -H, --host <HOST>                Host of the API to download from
    -m, --manifest <MANIFEST>        Path to manifest with file UUIDs to download
    -t, --token <TOKEN>              Auth token
    -T, --token-file <TOKEN_FILE>    Auth token file

ARGS:
    [UUIDS]...    File UUIDs to download
```


### Installation from source

First, install [Rust](https://github.com/rust-lang/rustup) and [Cargo](https://crates.io/).

```
git clone git@github.com:millerjs/gdc-client-rs.git
cd gdc-client-rs
cargo run
```
