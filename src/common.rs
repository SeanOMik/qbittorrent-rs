use serde_with::rust::seq_display_fromstr;

/// This module contains common structs, and functions that can be used
/// by other crates. This is re-exported in `abstracttorrent` and used in it.

#[derive(Debug, Clone)]
pub enum TorrentListFilter {
    All,
    Downloading,
    Seeding,
    Completed,
    Paused,
    Active,
    Inactive,
    Resumed,
    Stalled,
    StalledUploading,
    StalledDownloading,
    Errored,
}

impl TorrentListFilter {
    pub fn to_string(&self) -> &str {
        match *self {
            TorrentListFilter::All => "all",
            TorrentListFilter::Downloading => "downloading",
            TorrentListFilter::Seeding => "seeding",
            TorrentListFilter::Completed => "completed",
            TorrentListFilter::Paused => "paused",
            TorrentListFilter::Active => "active",
            TorrentListFilter::Inactive => "inactive",
            TorrentListFilter::Resumed => "resumed",
            TorrentListFilter::Stalled => "stalled",
            TorrentListFilter::StalledUploading => "stalled_uploading",
            TorrentListFilter::StalledDownloading => "stalled_downloading",
            TorrentListFilter::Errored => "errored",
        }
    }
}

#[derive(Default, Clone)]
pub struct GetTorrentListParams {
    /// Filter torrent list by state
    pub filter: Option<TorrentListFilter>,

    /// Get torrents with the given category 
    pub category: Option<String>,

    /// Get torrents with the given tag.
    pub tag: Option<String>,

    // TODO: Add `sort` support for TorrentInfo fields.

    /// Enable reverse sorting.
    pub reverse: Option<bool>,

    /// Limit the number of results.
    pub limit: Option<i32>,

    /// Set offset.
    pub offset: Option<i32>,

    /// Filter by hashes.
    pub hashes: Option<Vec<String>> // NOTE: Separated by `|`
}

impl GetTorrentListParams {
    pub fn builder() -> GetTorrentListParamsBuilder {
        GetTorrentListParamsBuilder::default()
    }

    pub fn to_params(&self) -> String {
        let mut params = String::new();

        if let Some(filter) = &self.filter {
            params.push_str(&format!("&filter={}", filter.to_string()));
        }

        if let Some(category) = &self.category {
            params.push_str(&format!("&category={}", category));
        }

        if let Some(tag) = &self.tag {
            params.push_str(&format!("&tag={}", tag));
        }

        if let Some(reverse) = &self.reverse {
            params.push_str(&format!("&reverse={}", reverse.to_string()));
        }

        if let Some(limit) = &self.limit {
            params.push_str(&format!("&limit={}", limit));
        }

        if let Some(offset) = &self.limit {
            params.push_str(&format!("&offset={}", offset));
        }

        if let Some(hashes) = &self.hashes {
            let hashes = hashes.join("|");
            params.push_str(&format!("&hashes={}", hashes));
        }

        params
    }
}

#[derive(Default)]
pub struct GetTorrentListParamsBuilder {
    param: GetTorrentListParams,
}

impl GetTorrentListParamsBuilder {
    /// Set a filter
    pub fn filter(&mut self, filter: TorrentListFilter) -> &mut Self {
        self.param.filter = Some(filter);

        self
    }

    /// Set a filter.
    pub fn category(&mut self, category: &str) -> &mut Self {
        self.param.category = Some(category.to_string());

        self
    }

    /// Set a tag.
    pub fn tag(&mut self, tag: &str) -> &mut Self {
        self.param.tag = Some(tag.to_string());

        self
    }

    /// Reverse the order of the results.
    pub fn reverse(&mut self) -> &mut Self {
        self.param.reverse = Some(true);

        self
    }

    /// Set a limit on the number of results returned.
    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.param.limit = Some(limit);

        self
    }

    /// Set an offset of the results.
    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.param.offset = Some(offset);

        self
    }

    /// Add a hash to filter by.
    pub fn hash(&mut self, hash: &str) -> &mut Self {
        self.param.hashes.as_mut()
            .unwrap_or(&mut vec![])
            .push(hash.to_string());

        self
    }

    /// Set the hashes to filter by.
    pub fn hashes(&mut self, hashes: Vec<String>) -> &mut Self {
        self.param.hashes = Some(hashes);

        self
    }

    pub fn build(&self) -> GetTorrentListParams {
        self.param.clone()
    }
}