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
    pub async fn login(&mut self, url: String, username: String, password: String) -> ClientResult<()> {
        // Send response to get auth string
        let resp = self.client.post(format!("{}/api/v2/auth/login", url.clone()))
            .form(&[
                ("username", username.clone()),
                ("password", password.clone()),
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
                url: url.clone(),
                username: username.clone(),
                password: password.clone(),
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

    pub async fn add_torrent(&self, upload: &TorrentUpload) -> ClientResult<()> {
        if let (Some(auth_string), Some(conn)) = (self.auth_string.as_ref(), self.connection_info.as_ref()) {
            // Construct and send request to qbittorrent
            let resp = self.client.post(format!("{}/api/v2/torrents/add", conn.url.clone()))
                .header(reqwest::header::COOKIE, auth_string.clone())
                .multipart(upload.to_multipart_form())
                .send().await?.error_for_status()?;

            Ok(())
        } else {
            Err(ClientError::Authorization)
        }
    }
}