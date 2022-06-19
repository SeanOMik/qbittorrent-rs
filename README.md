## Rust qBitorrent WebUI API implementation 
This crate is pretty bare bones. Its used in another one of my projects so I only implemented what I needed and nothing else. Although it was a bit rushed, I feel like its a good starting point in the case that it might be expanded on further. Feel free to submit PRs.

<br>

### TODO: ([qBittorrent API wiki](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-4.1)))
Authentication
- [x] Login
- [ ] Logout

Application
- [ ] Get application version
- [ ] Get API version
- [ ] Get build info
- [ ] Shutdown application
- [ ] Get application preferences
- [ ] Set application preferences
- [ ] Get default save path

Log
- [ ] Get log
- [ ] Get peer log

Sync
- [ ] Get main data
- [ ] Get torrent peers data

Transfer info
- [ ] Get global transfer info
- [ ] Get alternative speed limits state
- [ ] Toggle alternative speed limits
- [ ] Get global download limit
- [ ] Set global download limit
- [ ] Get global upload limit
- [ ] Set global upload limit
- [ ] Ban peers

Torrent management
- [x] Get torrent list
- [ ] Get torrent generic properties
- [x] Get torrent trackers
- [ ] Get torrent web seeds
- [ ] Get torrent contents
- [ ] Get torrent pieces' states
- [ ] Get torrent pieces' hashes
- [ ] Pause torrents
- [ ] Resume torrents
- [x] Delete torrents
- [ ] Recheck torrents
- [ ] Reannounce torrents
- [x] Edit trackers
- [x] Remove trackers
- [ ] Add peers
- [x] Add new torrent
- [x] Add trackers to torrent
- [ ] Increase torrent priority
- [ ] Decrease torrent priority
- [ ] Maximal torrent priority
- [ ] Minimal torrent priority
- [ ] Set file priority
- [ ] Get torrent download limit
- [ ] Set torrent download limit
- [ ] Set torrent share limit
- [ ] Get torrent upload limit
- [ ] Set torrent upload limit
- [ ] Set torrent location
- [ ] Set torrent name
- [ ] Set torrent category
- [ ] Get all categories
- [ ] Add new category
- [ ] Edit category
- [ ] Remove categories
- [ ] Add torrent tags
- [ ] Remove torrent tags
- [x] Get all tags
- [x] Create tags
- [x] Delete tags
- [ ] Set automatic torrent management
- [ ] Toggle sequential download
- [ ] Set first/last piece priority
- [ ] Set force start
- [ ] Set super seeding
- [ ] Rename file
- [ ] Rename folder

RSS (experimental)
- [ ] Add folder
- [ ] Add feed
- [ ] Remove item
- [ ] Move item
- [ ] Get all items
- [ ] Mark as read
- [ ] Refresh item
- [ ] Set auto-downloading rule
- [ ] Rename auto-downloading rule
- [ ] Remove auto-downloading rule
- [ ] Get all auto-downloading rules
- [ ] Get all articles matching a rule

Search
- [ ] Start search
- [ ] Stop search
- [ ] Get search status
- [ ] Get search results
- [ ] Delete search
- [ ] Get search plugins
- [ ] Install search plugin
- [ ] Uninstall search plugin
- [ ] Enable search plugin
- [ ] Update search plugins