extern crate clap;
use clap::Clap;
use varys::{index, merge, Document, InvertedIndex};

use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Collins Huff chuff@paloaltonetworks.com")]
struct Opts {
    /// Path to input file, defaults to stdin
    #[clap(short, long)]
    in_file: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct HttpHeader {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct HttpServer {
    tls_endpoint: TlsEndpoint,
    http_body: String,
    http_headers: Vec<HttpHeader>,
    http_version: String,
    http_status_code: u64,
    http_status_message: String,
    http_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DummyIP {
    ipv4: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TlsEndpoint {
    ip: DummyIP,
    port_number: u16,
    port_protocol: String,
    domain_name: String,
    server_name_indication_used: bool,
    start_tls_protocol: String,
}

impl From<HttpServer> for Document {
    fn from(http: HttpServer) -> Document {
        let host = if http.tls_endpoint.domain_name != "" {
            http.tls_endpoint.domain_name
        } else {
            http.tls_endpoint.ip.ipv4
        };

        let scheme = if http.tls_endpoint.port_number == 443 {
            "https"
        } else {
            "http"
        };

        let url = format!(
            "{}://{}:{}/{}",
            scheme, host, http.tls_endpoint.port_number, http.http_path
        );
        Document::new(url, http.http_body)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();
    env_logger::init();
    match &opts.in_file {
        Some(f) => {
            let f = File::open(&f)?;
            let b = std::io::BufReader::new(f);
            let index = read_http(b).expect("failed to read targets");
            for (k, v) in index {
                log::debug!("{:?}: {:?}", k, v);
            }
        }
        None => {
            let f = std::io::stdin();
            let b = std::io::BufReader::new(f);
            let index = read_http(b).expect("failed to read targets");
            for (k, v) in index {
                log::debug!("{:?}: {:?}", k, v);
            }
        }
    }

    Ok(())
}

pub fn read_http<R: std::io::Read>(
    reader: std::io::BufReader<R>,
) -> Result<InvertedIndex, Box<dyn Error>> {
    let mut full_index = InvertedIndex::new();
    for line in reader.lines() {
        let line = line?;
        let server: HttpServer = serde_json::from_str(&line)?;
        let doc = server.into();
        let cur_index = index(doc);
        full_index = merge(full_index, cur_index);
    }
    Ok(full_index)
}
