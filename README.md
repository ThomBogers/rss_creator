# Rss feed creator based on files with metadata in json

## todo
- Webserver
  - Fix streaming by enabling content_disposition,accept_ranges,206
  - Add nginx template when it is fully working
  - Try hosting using [caddy](https://caddyserver.com/tutorial/caddyfile)
- Common
  - Try to use basic auth, and use user:pass@url in feed
  - Use getters/setters for structs?
    - try [derive builder](https://docs.rs/derive_builder/0.7.2/derive_builder/)
  - Find out how to set 'global' values for file
    - try [lazy static](https://crates.io/crates/lazy_static)
  - Convert unrecoverable errors in lib to result values
- get_data
  - filter doesn't seem to fully work
## Data format
A dir named `data` containing a file named `channel.json` and `feed.json`

The file name `channel.json` should have the content:

```json
{
    "channel_id": "as25Agb123",
    "limit": 5
}
```

The file named `feed.json` should have the following content:

```json
{
    "url": "https://casts.com",
    "namespace": "mybussines",
    "category": "Business",
    "language": "nl",
    "title": "mybussines - the podcasts",
    "description": "a feed for podcasts pertaining to mybussines",
    "author": "Me",
    "email": "me@mybussines.com",
    "api_key": "secret",  
    "hide_from_store": true,
    "explicit": false
}
```


## Validate the feed
- [podba.se](https://podba.se/validate/)
- [castfeedvalidator.com](http://castfeedvalidator.com/find)
- [validator.w3.org](https://validator.w3.org/feed/check.cgi)
