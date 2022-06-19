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

    let torrents = client.get_torrent_list().await.unwrap();
    
    let first = torrents.first().unwrap();

    client.get_torrent_trackers(first).await.unwrap();

    println!("Hello, world!");
}
