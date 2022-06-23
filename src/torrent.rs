use serde::{Serialize, Deserialize};
use serde_repr::*;
use serde_with::{CommaSeparator};

/// A torrent's information from the qbittorrent client.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TorrentInfo {
    /// Time (Unix Epoch) when the torrent was added to the client
    pub added_on: u64,

    /// Amount of data left to download (bytes)
    pub amount_left: u64,

    /// Whether this torrent is managed by Automatic Torrent Management
    pub auto_tmm: bool,

    /// Percentage of file pieces currently available
    pub availability: f32,

    /// Category of the torrent
    pub category: String,

    /// Amount of transfer data completed (bytes)
    pub completed: u64,

    /// Time (Unix Epoch) when the torrent completed
    pub completion_on: u64,

    /// Absolute path of torrent content (root path for multi-file torrents, absolute file path for single-file torrents)
    pub content_path: String,

    /// Torrent download speed limit (bytes/s). -1 if unlimited.
    pub dl_limit: i64,

    /// Torrent download speed (bytes/s)
    pub dlspeed: u64,

    /// Amount of data downloaded
    pub downloaded: u64,

    /// Amount of data downloaded this session
    pub downloaded_session: u64,

    /// Torrent ETA (seconds)
    pub eta: i64,

    /// True if first last piece are prioritized
    pub f_l_piece_prio: bool,

    /// True if force start is enabled for this torrent
    pub force_start: bool,

    /// Torrent hash
    pub hash: String,

    /// Last time (Unix Epoch) when a chunk was downloaded/uploaded
    pub last_activity: i64,

    /// Magnet URI corresponding to this torrent
    pub magnet_uri: String,

    /// Maximum share ratio until torrent is stopped from seeding/uploading
    pub max_ratio: f32,

    /// Maximum seeding time (seconds) until torrent is stopped from seeding
    pub max_seeding_time: i32,

    /// Torrent name
    pub name: String,

    /// Number of seeds in the swarm
    pub num_complete: i32,

    /// Number of leechers in the swarm
    pub num_incomplete: i32,

    /// Number of leechers connected to
    pub num_leechs: i32,

    /// Number of seeds connected to
    pub num_seeds: i32,

    /// Torrent priority. Returns -1 if queuing is disabled or torrent is in seed mode
    pub priority: i32,

    /// Torrent progress (percentage/100)
    pub progress: f32,

    /// Torrent share ratio. Max ratio value: 9999.
    pub ratio: f32,

    pub ratio_limit: f32,
    
    /// Path where this torrent's data is stored
    pub save_path: String,

    /// Torrent elapsed time while complete (seconds)
    pub seeding_time: i32,

    /// per torrent setting, when Automatic Torrent Management is disabled,
    /// furthermore then max_seeding_time is set to seeding_time_limit for this
    /// torrent. If Automatic Torrent Management is enabled, the value is -2. And if
    /// max_seeding_time is unset it have a default value -1.
    pub seeding_time_limit: i32,

    /// Time (Unix Epoch) when this torrent was last seen complete
    pub seen_complete: i32,

    /// True if sequential download is enabled
    pub seq_dl: bool,

    /// Total size (bytes) of files selected for download
    pub size: i64,

    /// Torrent state. See table here below for the possible values
    pub state: TorrentState,

    /// True if super seeding is enabled
    pub super_seeding: bool,

    /// Tag list of the torrent
    #[serde(with = "serde_with::rust::StringWithSeparator::<CommaSeparator>")]
    pub tags: Vec<String>,

    /// Total active time (seconds)
    pub time_active: i32,

    /// Total size (bytes) of all file in this torrent (including unselected ones)
    pub total_size: i64,

    /// The first tracker with working status. Returns empty string if no tracker is working.
    pub tracker: String,

    /// Torrent upload speed limit (bytes/s). -1 if unlimited.
    pub up_limit: i64,

    /// Amount of data uploaded
    pub uploaded: u64,

    /// Amount of data uploaded this session
    pub uploaded_session: u64,

    /// Torrent upload speed (bytes/s)
    pub upspeed: u64,
}

/// An enum representing the state of a torrent in the client.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum TorrentState {
    /// Some error occurred, applies to paused torrents
    #[serde(rename = "error")]
    Error,

    /// Torrent data files is missing
    #[serde(rename = "missingFiles")]
    MissingFiles,

    /// Torrent is being seeded and data is being transferred
    #[serde(rename = "uploading")]
    Uploading,

    /// Torrent is paused and has finished downloading
    #[serde(rename = "pausedUP")]
    PausedUP,

    /// Queuing is enabled and torrent is queued for upload
    #[serde(rename = "queuedUP")]
    QueuedUP,

    /// Torrent is being seeded, but no connection were made
    #[serde(rename = "stalledUP")]
    StalledUP,

    /// Torrent has finished downloading and is being checked
    #[serde(rename = "checkingUP")]
    CheckingUP,

    /// Torrent is forced to uploading and ignore queue limit
    #[serde(rename = "forcedUP")]
    ForcedUP,

    /// Torrent is allocating disk space for download
    #[serde(rename = "allocating")]
    Allocating,

    /// Torrent is being downloaded and data is being transferred
    #[serde(rename = "downloading")]
    Downloading,

    /// Torrent has just started downloading and is fetching metadata
    #[serde(rename = "metaDL")]
    MetaDownloading,

    /// Torrent is paused and has NOT finished downloading
    #[serde(rename = "pausedDL")]
    PausedDL,

    /// Queuing is enabled and torrent is queued for download
    #[serde(rename = "queuedDL")]
    QueuedDL,

    /// Torrent is being downloaded, but no connection were made
    #[serde(rename = "stalledDL")]
    StalledDL,

    /// Same as checkingUP, but torrent has NOT finished downloading
    #[serde(rename = "checkingDL")]
    CheckingDL,

    /// Torrent is forced to downloading to ignore queue limit
    #[serde(rename = "forcedDL")]
    ForcedDL,

    /// Checking resume data on qBt startup
    #[serde(rename = "checkingResumeData")]
    CheckingResumeData,

    /// Torrent is moving to another location
    #[serde(rename = "moving")]
    Moving,

    /// Unknown status
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for TorrentState {
    fn default() -> Self {
        TorrentState::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentTracker {
    /// Tracker URL
    pub url: String,

    /// Tracker status. See the table below for possible values
    pub status: TrackerStatus,

    /// Tracker priority tier. Lower tier trackers are tried before higher
    /// tiers. Tier numbers are valid when >= 0, < 0 is used as placeholder
    /// when tier does not exist for special entries (such as DHT).
    pub tier: i32, 

    /// Number of peers for current torrent, as reported by the tracker
    pub num_peers: i32,

    /// Number of seeds for current torrent, as reported by the tracker
    pub num_seeds: i32,

    /// Number of leeches for current torrent, as reported by the tracker
    pub num_leeches: i32,

    /// Number of completed downloads for current torrent, as reported by the tracker
    pub num_downloaded: i32,

    /// Tracker message (there is no way of knowing what this message is - it's up to tracker admins)
    #[serde(rename = "msg")]
    pub message: String,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum TrackerStatus {
    /// Tracker is disabled (used for DHT, PeX, and LSD)
    Disabled = 0,

    /// Tracker has not been contacted yet
    NotContacted = 1,
    
    /// Tracker has been contacted and is working
    Working = 2,
    
    /// Tracker is updating
    Updating = 3,
    
    /// Tracker has been contacted, but it is not working (or doesn't send proper replies)
    NotWorking = 4
}

/// Represents a request to add torrents to the client.
#[derive(Debug, Default/* , Serialize, Deserialize */)]
pub struct TorrentUpload {
    /// URL(s) of the torrent files. When specifying `http` or `https` URLs, they
    /// don't always get downloaded by qbittorrent. The best way to verify if it was added
    /// to the client is to check the torrent list after the request.
    pub urls: Vec<String>, // NOTE: Separated by new lines

    /// Binary data of the torrents that are being added.
    /// Torrent file data that is being added. (Name, Bytes)
    pub torrents: Vec<(String, Vec<u8>)>,

    /// Download folder
    pub save_path: Option<String>, // NOTE: Rename to `savepath` for (de)serialization

    /// Cookie sent to download the .torrent file
    pub cookie: Option<String>,

    /// Category for the torrent
    pub category: Option<String>,

    /// Tags for the torrent
    pub tags: Option<Vec<String>>, // NOTE: Split by commas

    /// Skip hash checking.
    pub skip_hash_check: Option<bool>, // NOTE: Convert to string and rename to `skip_hash_check` for (de)serialization

    /// Add torrents in the paused state.
    pub paused: Option<bool>,

    /// Create the root folder.
    pub root_folder: Option<bool>, // NOTE: Convert to string for (de)serialization

    /// Rename torrent
    pub rename: Option<String>,

    /// Set torrent upload speed limit. Unit in bytes/second
    pub upload_limit: Option<i64>, // NOTE: Rename to `upLimit` for (de)serialization

    /// Set torrent download speed limit. Unit in bytes/second
    pub download_limit: Option<i64>, // NOTE: Rename to `upLimit` for (de)serialization

    /// Set torrent share ratio limit
    pub ratio_limit: Option<f32>, // NOTE: Rename to `ratioLimit` for (de)serialization

    /// Set torrent seeding time limit. Unit in seconds
    pub seeding_time_limit: Option<u64>, // NOTE: Rename to `seedingTimeLimit` for (de)serialization

    /// Whether Automatic Torrent Management should be used
    pub auto_tmm: Option<bool>, // NOTE: Rename to `autoTMM` for (de)serialization

    /// Enable sequential download. Possible values are true, false (default)
    pub sequential_download: Option<bool>, // NOTE: Rename to `sequentialDownload` and convert to string for (de)serialization

    /// Prioritize download first last piece. Possible values are true, false (default)
    pub first_last_piece_prio: Option<bool>, // NOTE: Rename to `firstLastPiecePrio` and convert to string for (de)serialization
}

#[derive(Debug, Default)]
pub struct TorrentUploadBuilder {
    params: TorrentUpload,
}

impl TorrentUploadBuilder {
    pub fn url(&mut self, url: String) -> &mut Self {
        self.params.urls.push(url);
        self
    }

    pub fn torrent_file(&mut self, torrent_path: String) -> &mut Self {
        let path = std::path::Path::new(&torrent_path);
        
        self.torrent_path(path)
    }

    pub fn torrent_path(&mut self, torrent_path: &std::path::Path) -> &mut Self {
        let torrents = &mut self.params.torrents;
        torrents.push((
            torrent_path.file_name().unwrap().to_str().unwrap().to_string(),
            std::fs::read(torrent_path).unwrap(),
        ));
        
        self
    }

    pub fn torrent_data(&mut self, filename: String, data: Vec<u8>) -> &mut Self {
        let torrents = &mut self.params.torrents;
        torrents.push((
            filename,
            data,
        ));
        
        self
    }

    pub fn save_path(&mut self, save_path: String) -> &mut Self {
        self.params.save_path = Some(save_path);
        self
    }

    pub fn cookie(&mut self, cookie: String) -> &mut Self {
        self.params.cookie = Some(cookie);
        self
    }

    pub fn category(&mut self, category: String) -> &mut Self {
        self.params.category = Some(category);
        self
    }

    pub fn tag(&mut self, tag: String) -> &mut Self {
        self.params.tags.as_mut().unwrap_or(&mut vec![]).push(tag);
        self
    }

    pub fn tags(&mut self, tags: Vec<String>) -> &mut Self {
        self.params.tags = Some(tags);
        self
    }

    pub fn skip_hash_check(&mut self, skip_hash_check: bool) -> &mut Self {
        self.params.skip_hash_check = Some(skip_hash_check);
        self
    }

    pub fn paused(&mut self, paused: bool) -> &mut Self {
        self.params.paused = Some(paused);
        self
    }

    pub fn root_folder(&mut self, root_folder: bool) -> &mut Self {
        self.params.root_folder = Some(root_folder);
        self
    }

    pub fn rename(&mut self, rename: String) -> &mut Self {
        self.params.rename = Some(rename);
        self
    }

    pub fn upload_limit(&mut self, upload_limit: i64) -> &mut Self {
        self.params.upload_limit = Some(upload_limit);
        self
    }

    pub fn download_limit(&mut self, download_limit: i64) -> &mut Self {
        self.params.download_limit = Some(download_limit);
        self
    }

    pub fn ratio_limit(&mut self, ratio_limit: f32) -> &mut Self {
        self.params.ratio_limit = Some(ratio_limit);
        self
    }

    pub fn seeding_time_limit(&mut self, seeding_time_limit: u64) -> &mut Self {
        self.params.seeding_time_limit = Some(seeding_time_limit);
        self
    }

    pub fn auto_tmm(&mut self, auto_tmm: bool) -> &mut Self {
        self.params.auto_tmm = Some(auto_tmm);
        self
    }

    pub fn sequential_download(&mut self, sequential_download: bool) -> &mut Self {
        self.params.sequential_download = Some(sequential_download);
        self
    }

    pub fn first_last_piece_prio(&mut self, first_last_piece_prio: bool) -> &mut Self {
        self.params.first_last_piece_prio = Some(first_last_piece_prio);
        self
    }

    pub fn build(&self) -> &TorrentUpload {
        &self.params
    }
}

impl TorrentUpload {
    /// Get a builder of `TorrentUpload`
    pub fn builder() -> TorrentUploadBuilder {
        TorrentUploadBuilder::default()
    }

    // TODO: Add result for when neither `urls` and `torrents` are not set. For now it just panics.
    pub fn to_multipart_form(&self) -> reqwest::multipart::Form {
        if self.urls.is_empty() && self.torrents.is_empty() {
            panic!("Either `urls` or `torrents` must be set!!");
        }

        let mut form = reqwest::multipart::Form::new();
        
        // Add urls separated by new lines
        if !self.urls.is_empty() {
            let urls = self.urls.join("\n");

            form = form.text("urls", urls); // For some reason I have to do this :(
        }

        // Add the torrents as files
        if !self.torrents.is_empty() {
            for torrent in self.torrents.iter() {
                // TODO: Avoid a clone here?
                form = form.part("torrents", reqwest::multipart::Part::bytes(torrent.1.clone())
                    .file_name(torrent.0.clone())
                    .mime_str("application/x-bittorrent").unwrap());
            }
        }

        if let Some(save_path) = &self.save_path {
            form = form.text("savepath", save_path.to_owned());
        }

        if let Some(cookie) = &self.cookie {
            form = form.text("cookie", cookie.to_owned());
        }

        if let Some(category) = &self.category {
            form = form.text("category", category.to_owned());
        }

        if let Some(tags) = &self.tags {
            let tags = tags.join(",");
            form = form.text("tags", tags);
        }

        if let Some(skip_hash_check) = &self.skip_hash_check {
            form = form.text("skip_checking", skip_hash_check.to_string());
        }

        if let Some(paused) = &self.paused {
            form = form.text("paused", paused.to_string());
        }

        if let Some(root_folder) = &self.root_folder {
            form = form.text("root_folder", root_folder.to_string());
        }

        if let Some(rename) = &self.rename {
            form = form.text("rename", rename.to_owned());
        }

        if let Some(upload_limit) = &self.upload_limit {
            form = form.text("upLimit", upload_limit.to_string());
        }

        if let Some(download_limit) = &self.download_limit {
            form = form.text("dlLimit", download_limit.to_string());
        }

        if let Some(ratio_limit) = &self.ratio_limit {
            form = form.text("ratioLimit", ratio_limit.to_string());
        }

        if let Some(seeding_time_limit) = &self.seeding_time_limit {
            form = form.text("seedingTimeLimit", seeding_time_limit.to_string());
        }

        if let Some(auto_tmm) = &self.auto_tmm {
            form = form.text("autoTMM", auto_tmm.to_string());
        }

        if let Some(sequential_download) = &self.sequential_download {
            form = form.text("sequentialDownload", sequential_download.to_string());
        }

        if let Some(first_last_piece_prio) = &self.first_last_piece_prio {
            form = form.text("firstLastPiecePrio", first_last_piece_prio.to_string());
        }

        form
    }
}