mod args;

use args::Args;
use clap::Parser;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use futures::stream;
use futures::StreamExt;
use reqwest::header::HeaderMap;
use reqwest::Client;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::Path;

enum Ext {
    Known(String),
    Unknown(String),
}

fn read_lines(filename: &str) -> Vec<(u64, String)> {
    let mut result = Vec::new();
    let mut c = 1;
    for line in read_to_string(filename).unwrap().lines() {
        result.push((c, line.to_string()));
        c = c + 1;
    }
    result
}

fn resolve_extension(headers: &HeaderMap) -> Option<Ext> {
    let cth = headers.get("content-type")?;
    let cts = cth.to_str();
    if let Ok(content_type) = cts {
        match content_type {
            "image/jpeg" => Some(Ext::Known("jpg".to_string())),
            "image/png" => Some(Ext::Known("png".to_string())),
            "image/gif" => Some(Ext::Known("gif".to_string())),
            _ => Some(Ext::Unknown(content_type.to_string())),
        }
    } else {
        None
    }
}

fn gen_filename(dir: &String, i: u64, url: &String, ext: String) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(url);
    let hex = hasher.result_str();
    let filename = format!("{}-{}.{}", i, hex, ext);
    let filewithdir = Path::new(dir).join(filename).to_str().unwrap().to_string();
    filewithdir
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let urls = read_lines(&args.file);
    let client = Client::new();
    create_dir_all(&args.outdir)?;

    stream::iter(urls)
        .for_each_concurrent(args.threads, |(i, url)| {
            let client = &client;
            let outdir = args.outdir.clone();
            async move {
                match client.get(&url).send().await {
                    Ok(response) => {
                        if response.status() == 200 {
                            match resolve_extension(response.headers()) {
                                Some(Ext::Known(ext)) => {
                                    let filename = gen_filename(&outdir, i, &url, ext);
                                    match response.bytes().await {
                                        Ok(bytes) => match write(&filename, bytes) {
                                            Ok(_) => {}
                                            Err(e) => println!(
                                                "error for url {} on writting file {}: {}",
                                                url, &filename, e
                                            ),
                                        },
                                        Err(e) => println!("error for url {}: {}", url, e),
                                    }
                                }
                                Some(Ext::Unknown(content_type)) => {
                                    println!(
                                        "error for url {}: unknown content-type {}",
                                        url, content_type
                                    );
                                }
                                None => {
                                    println!("error for url {}: couldn't resolve extension", url)
                                }
                            }
                        } else {
                            println!(
                                "error for url {}: response code is {}",
                                url,
                                response.status()
                            );
                        }
                    }
                    Err(e) => {
                        println!("error for url {}: {}", url, e);
                    }
                }
            }
        })
        .await;
    Ok(())
}
