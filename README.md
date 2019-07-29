# Rss feed creator based on files with metadata in json

## todo
- Nginx
  - Fix streaming by enabling content_disposition,accept_ranges,206
  - Add nginx template when it is fully working
- Common
  - Try to use basic auth, and use user:pass@url in feed
  - Use getters/setters for structs?
  - Find out how to set 'global' values for file
    - https://crates.io/crates/lazy_static
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
- https://podba.se/validate/
- http://castfeedvalidator.com/find
- https://validator.w3.org/feed/check.cgi