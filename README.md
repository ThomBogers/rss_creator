# Rss feed creator based on files with metadata in json

## todo
- Remove all instances of unwrap

## Data format
A dir named `data` containing a file named `casts.json`, `meta.json` and the cast files

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

The file named `meta.json` should have the following content:

```json
{
    "url": "http://casts.com",
    "category": "Business"
}
```


## Validate the feed
- https://podba.se/validate/
- http://castfeedvalidator.com/find
