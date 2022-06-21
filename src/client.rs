use crate::{error::ClientError, TorrentInfo, TorrentTracker, TorrentUpload};

pub struct ConnectionInfo {
    pub url: String,
    pub username: String,
    pub password: String,
}

pub type ClientResult<T> = Result<T, ClientError>;

pub struct QBittorrentClient {
    client: reqwest::Client,
    connection_info: Option<ConnectionInfo>,
    auth_string: Option<String>,
}

impl QBittorrentClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            connection_info: None,
            auth_string: None,
        }
    }

    /// Login to qBittorrent. This must be ran so that the client can make requests.
    pub async fn login(&mut self, url: &str, username: &str, password: &str) -> ClientResult<()> {
        // Send response to get auth string
        let resp = self.client.post(format!("{}/api/v2/auth/login", url))
            .form(&[
                ("username", username.to_string()),
                ("password", password.to_string()),
            ])
            .send().await?.error_for_status()?;

        let headers = resp.headers().clone();
        let content = resp.text().await?;

        if content == "Ok." {
            // Extract cookies
            let cookies: Vec<_> = headers.get(reqwest::header::SET_COOKIE)
                .unwrap().to_str().unwrap().split(';').collect();

            // Extract auth string and store it.
            let auth_string = cookies.iter().find(|c| c.starts_with("SID=")).unwrap();
            self.auth_string = Some(auth_string.to_string());

            // Store connection info
            self.connection_info = Some(ConnectionInfo {
                url: url.to_string(),
                username: username.to_string(),
                password: password.to_string(),
            });

            Ok(())
        } else {
            Err(ClientError::Authorization)
        }
    }

    /// Get a list of all torrents in the client.
    pub async fn get_torrent_list(&self) -> ClientResult<Vec<TorrentInfo>> {
        if let (Some(auth_string), Some(conn)) = (self.auth_string.as_ref(), self.connection_info.as_ref()) {
            // Construct and send request to qbittorrent
            let resp = self.client.post(format!("{}/api/v2/torrents/info", conn.url.clone()))
                .header(reqwest::header::COOKIE, auth_string.clone())
                .send().await?.error_for_status()?;

            // Deserialize response
            let content = resp.text().await?;
            let torrents: Vec<TorrentInfo> = serde_json::from_str(&content)?;

            Ok(torrents)
        } else {
            Err(ClientError::Authorization)
        }
    }

    /// Get a list of trackers for a torrent.
    pub async fn get_torrent_trackers(&self, torrent: &TorrentInfo) -> ClientResult<Vec<TorrentTracker>> {
        if let (Some(auth_string), Some(conn)) = (self.auth_string.as_ref(), self.connection_info.as_ref()) {
            // Construct and send request to qbittorrent
            let resp = self.client.post(format!("{}/api/v2/torrents/trackers", conn.url.clone()))
                .header(reqwest::header::COOKIE, auth_string.clone())
                .form(&[
                    ("hash", torrent.hash.clone()),
                ])
                .send().await?.error_for_status()?;

            // Deserialize response
            let content = resp.text().await?;
            let trackers: Vec<TorrentTracker> = serde_json::from_str(&content)?;

            Ok(trackers)
        } else {
            Err(ClientError::Authorization)
        }
    }

    /// Add a tracker to a torrent.
    pub async fn add_torrent_tracker(&self, torrent: &TorrentInfo, tracker_url: String) -> ClientResult<()> {
        if let (Some(auth_string), Some(conn)) = (self.auth_string.as_ref(), self.connection_info.as_ref()) {
            // Construct and send request to qbittorrent
            let _resp = self.client.post(format!("{}/api/v2/torrents/addTrackers", conn.url.clone()))
                .header(reqwest::header::COOKIE, auth_string.clone())
                .form(&[
                    ("hash", torrent.hash.clone()),
                    ("urls", tracker_url),
                ])
                .send().await?.error_for_status()?;

            Ok(())
        } else {
            Err(ClientError::Authorization)
        }
    }

    /// Replace a tracker url on a torrent.
    pub async fn replace_torrent_tracker(&self, torrent: &TorrentInfo, old_url: String, new_url: String) -> ClientResult<()> {
        if let (Some(auth_string), Some(conn)) = (self.auth_string.as_ref(), self.connection_info.as_ref()) {
            // Construct and send request to qbittorrent
            let _resp = self.client.post(format!("{}/api/v2/torrents/editTracker", conn.url.clone()))
                .header(reqwest::header::COOKIE, auth_string.clone())
                .form(&[
                    ("hash", torrent.hash.clone()),
                    ("origUrl", old_url),
                    ("newUrl", new_url),
                ])
                .send().await?.error_for_status()?;

            Ok(())
        } else {
            Err(ClientError::Authorization)
        }
    }

    /// Remove a tracker url on a torrent.
    pub async fn remove_torrent_tracker(&self, torrent: &TorrentInfo, tracker_url: String) -> ClientResult<()> {
        if let (Some(auth_string), Some(conn)) = (self.auth_string.as_ref(), self.connection_info.as_ref()) {
            // Construct and send request to qbittorrent
            let _resp = self.client.post(format!("{}/api/v2/torrents/removeTrackers", conn.url.clone()))
                .header(reqwest::header::COOKIE, auth_string.clone())
                .form(&[
                    ("hash", torrent.hash.clone()),
                    ("urls", tracker_url),
                ])
                .send().await?.error_for_status()?;

            Ok(())
        } else {
            Err(ClientError::Authorization)
        }
    }

    pub async fn add_torrent(&self, upload: &TorrentUpload) -> ClientResult<()> {
        if let (Some(auth_string), Some(conn)) = (self.auth_string.as_ref(), self.connection_info.as_ref()) {
            // Construct and send request to qbittorrent
            let _resp = self.client.post(format!("{}/api/v2/torrents/add", conn.url.clone()))
                .header(reqwest::header::COOKIE, auth_string.clone())
                .multipart(upload.to_multipart_form())
                .send().await?.error_for_status()?;

            Ok(())
        } else {
            Err(ClientError::Authorization)
        }
    }

    /// Remove a torrent from the client.
    pub async fn remove_torrent(&self, torrent: &TorrentInfo, delete_files: bool) -> ClientResult<()> {
        if let (Some(auth_string), Some(conn)) = (self.auth_string.as_ref(), self.connection_info.as_ref()) {
            // Construct and send request to qbittorrent
            let _resp = self.client.post(format!("{}/api/v2/torrents/delete", conn.url.clone()))
                .header(reqwest::header::COOKIE, auth_string.clone())
                .form(&[
                    ("hashes", torrent.hash.clone()),
                    ("deleteFiles", delete_files.to_string()),
                ]).send().await?.error_for_status()?;


            Ok(())
        } else {
            Err(ClientError::Authorization)
        }
    }

    /// Remove multiple torrents at once. `delete_files` applies to *all* torrents.
    pub async fn remove_torrents(&self, torrents: Vec<TorrentInfo>, delete_files: bool ) -> ClientResult<()> {
        if let (Some(auth_string), Some(conn)) = (self.auth_string.as_ref(), self.connection_info.as_ref()) {
            // Convert the hashes into a string concatenated with `|`
            let hashes = torrents.iter()
                .map(|t| t.hash.clone())
                .collect::<Vec<_>>()
                .join("|");

            // Construct and send request to qbittorrent
            let _resp = self.client.post(format!("{}/api/v2/torrents/delete", conn.url.clone()))
                .header(reqwest::header::COOKIE, auth_string.clone())
                .form(&[
                    ("hashes", hashes),
                    ("deleteFiles", delete_files.to_string()),
                ]).send().await?.error_for_status()?;
            Ok(())
        } else {
            Err(ClientError::Authorization)
        }
    }

    /// Get all tags
    pub async fn get_tags(&self) -> ClientResult<Vec<String>> {
        if let (Some(auth_string), Some(conn)) = (self.auth_string.as_ref(), self.connection_info.as_ref()) {
            // Construct and send request to qbittorrent
            let resp = self.client.get(format!("{}/api/v2/torrents/tags", conn.url.clone()))
                .header(reqwest::header::COOKIE, auth_string.clone())
                .send().await?.error_for_status()?;

            // Deserialize response
            let content = resp.text().await?;
            let tags: Vec<String> = serde_json::from_str(&content)?;

            Ok(tags)
        } else {
            Err(ClientError::Authorization)
        }
    }

    /// Create a new tag
    pub async fn create_tag(&self, tag: &str) -> ClientResult<()> {
        if let (Some(auth_string), Some(conn)) = (self.auth_string.as_ref(), self.connection_info.as_ref()) {
            // Construct and send request to qbittorrent
            let _resp = self.client.post(format!("{}/api/v2/torrents/createTags", conn.url.clone()))
                .header(reqwest::header::COOKIE, auth_string.clone())
                .form(&[
                    ("tags", tag),
                ]).send().await?.error_for_status()?;

            Ok(())
        } else {
            Err(ClientError::Authorization)
        }
    }

    /// Delete a tag
    pub async fn delete_tag(&self, tag: &str) -> ClientResult<()> {
        if let (Some(auth_string), Some(conn)) = (self.auth_string.as_ref(), self.connection_info.as_ref()) {
            // Construct and send request to qbittorrent
            let _resp = self.client.post(format!("{}/api/v2/torrents/deleteTags", conn.url.clone()))
                .header(reqwest::header::COOKIE, auth_string.clone())
                .form(&[
                    ("tags", tag),
                ]).send().await?.error_for_status()?;

            Ok(())
        } else {
            Err(ClientError::Authorization)
        }
    }
}