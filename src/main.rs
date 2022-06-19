/// NOTE: USED FOR TESTING

pub mod torrent;
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
