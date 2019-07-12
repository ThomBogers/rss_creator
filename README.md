# Rss feed creator based on files with metadata in json

## Data format
A dir named `data` containing a file named `casts.json` and the cast files

The file named `casts.json`, should have the following content:

```json
[
    {
        "filename":"foo.m4a",
        "episodename": "foo or bar",
        "author": "bar foo",
        "created_at": "12/3/2019"
    }
]
```


## Validate the feed
- https://podba.se/validate/
- http://castfeedvalidator.com/find
