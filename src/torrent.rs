use serde::{Serialize, Deserialize};
use serde_repr::*;
use serde_with::{CommaSeparator};

/// A torrent's information from the qbittorrent client.
#[derive(Debug, Serialize, Deserialize)]
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
    pub eta: u64,

    /// True if first last piece are prioritized
    pub f_l_piece_prio: bool,

    /// True if force start is enabled for this torrent
    pub force_start: bool,

    /// Torrent hash
    pub hash: String,

    /// Last time (Unix Epoch) when a chunk was downloaded/uploaded
    pub last_activity: u64,

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
    pub size: u64,

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
    pub total_size: u64,

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
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
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

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
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