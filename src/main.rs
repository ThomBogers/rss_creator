use std::fs;
use std::fs::File;
use std::io::prelude::*;

use rss;

use serde::Serialize;
use serde::Deserialize;
use serde_json;


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Cast {
    filename: String,
    episodename: String,
    author: String,
    created_at: String
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Metadata {
    url: String,
    category: String,
}

fn main() {
    println!("Start rss builder!");

    let file = fs::File::open("./data/casts.json")
        .expect("file should open read only");

    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");
    
    let data = json.as_array()
        .expect("file should contain JSON array");

    let mut channel = rss::ChannelBuilder::default()
        .title("Ksyos kennis sessies")
        .link(get_metadata().url)
        .description("An RSS feed.")
        .build()
        .unwrap();
    
    let mut items = channel.items().to_vec();

    for index in 0..data.len() {
        let item = data.get(index).unwrap().clone();
        let cast: Cast = serde_json::from_value(item).unwrap();
        let feed_item = get_feed_item(cast);
        items.push(feed_item);
    }

    channel.set_items(items);
    let xml = channel.to_string();
    write_feed(&xml);
    println!("Result:\n{}", xml);
}

fn write_feed(feed: &str) {
    let mut f = File::create("./data/rss.xml").unwrap();
    write!(f, "{}", feed).unwrap();
    f.sync_all().unwrap();
}

fn get_feed_item(cast: Cast) -> rss::Item {
    println!("\tget feed item for cast: {:?}", cast);

    let file_meta = fs::metadata(format!("./data/{}", cast.filename))
        .expect(&format!("Could not open file ./data/{}", cast.filename));

    let file_size = file_meta.len();

    let enclosure = rss::EnclosureBuilder::default()
        .url(format!("{}/casts/{}", get_metadata().url, cast.filename))
        .length(format!("{}", file_size))
        .mime_type("m4a")
        .build()
        .unwrap();

    let mut category = rss::Category::default();
    category.set_name(get_metadata().category);

    let categories = vec!(category);

    rss::ItemBuilder::default()
        .link(format!("{}/casts/{}", get_metadata().url, cast.filename))
        .title(cast.episodename)
        .author(cast.author)
        .pub_date(cast.created_at)
        .enclosure(enclosure)
        .categories(categories)
        .build()
        .unwrap()
}

fn get_metadata() -> Metadata{
    let file = fs::File::open("./data/meta.json")
        .expect("file should open read only");
    
    let meta: Metadata = serde_json::from_reader(file).expect("Could not parse metadata");
    meta
}