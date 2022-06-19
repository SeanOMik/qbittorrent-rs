#[macro_use]
extern crate derive_builder;

pub mod torrent;
use std::path::Path;

pub use torrent::*;

pub mod client;
pub use client::*;

pub mod error;
pub use error::*;

#[tokio::main]
async fn main() {
    let mut client = QBittorrentClient::new();

    client.login(
        String::from("http://localhost:8080"),
        String::from("admin"),
        String::from("adminadmin")
    ).await.unwrap();

    println!("Hello, world!");
}
