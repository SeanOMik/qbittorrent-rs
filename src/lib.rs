pub mod torrent;
pub mod client;
pub mod error;
pub mod common;

#[cfg(test)]
mod tests {
    macro_rules! block_on {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_login() {
        let mut client = super::client::QBittorrentClient::new();

        block_on!(client.login("http://localhost:8080", "admin", "adminadmin")).unwrap();

        println!("Logged in!");
    }
}