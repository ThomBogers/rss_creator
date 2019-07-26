use std::fs;
use std::fs::File;
use std::io::prelude::*;

use rss;
use rss::extension::itunes::{ITunesChannelExtension, ITunesOwner};

use serde::Serialize;
use serde::Deserialize;
use serde_json;

use std::collections::HashMap;

use rss_creator::{Cast};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Metadata {
    url: String,
    namespace: String,
    category: String,
    language: String,
    title: String,
    description: String,
    author: String,
    email: String,
    api_key: String,
    explicit: bool,
    hide_from_store: bool,
}

fn main() {
    println!("Starting rss builder!");

    let casts = get_cast_data();
    let feed_items: Vec<rss::Item> = casts
        .iter()
        .map(get_feed_item)
        .collect();

    let mut channel = get_channel();
    channel.set_items(feed_items);

    let xml = channel.to_string();
    write_feed(&xml);
    println!("Done");
}

fn write_feed(feed: &str) {
    let mut f = File::create("./data/rss.xml")
        .expect("Could not create rss.xml");

    write!(f, "{}", feed).unwrap();
    f.sync_all().unwrap();
}

fn get_data_link(filename: &str, namespaced: bool) -> String{
    let metadata = get_metadata();

    let url = if namespaced {
        format!("{}/{}", metadata.url, metadata.namespace)
    } else {
        metadata.url
    };

    let api_key = &metadata.api_key;
    if api_key.len() > 0 {
        format!("{}/{}?auth={}", url, filename, api_key)
    } else {
        format!("{}/{}", url, filename)
    }
}

fn get_feed_item(cast: &Cast) -> rss::Item {
    println!("\tget feed item for cast: {:?}", cast);

    let file_meta = fs::metadata(format!("./data/{}", cast.filename))
        .expect(&format!("Could not open file ./data/{}", cast.filename));

    let file_size = file_meta.len();

    let enclosure = rss::EnclosureBuilder::default()
        .url(get_data_link(&cast.filename, true))
        .length(format!("{}", file_size))
        .mime_type("audio/mpeg")
        .build()
        .unwrap();

    rss::ItemBuilder::default()
        .link(get_data_link(&cast.filename, true))
        .title(cast.title.clone())
        .author(cast.author.clone())
        .pub_date(cast.created_at.clone())
        .enclosure(enclosure)
        .itunes_ext(rss::extension::itunes::ITunesItemExtension::default())
        .dublin_core_ext(rss::extension::dublincore::DublinCoreExtension::default())
        .build()
        .unwrap()
}

fn get_channel() -> rss::Channel {
    let metadata = get_metadata();

    let mut itunes_extension = ITunesChannelExtension::default();
    itunes_extension.set_author(format!("{}", metadata.author));
    itunes_extension.set_image(get_data_link(&format!("{}.png", metadata.namespace), false));
    itunes_extension.set_block( if metadata.hide_from_store {"Yes".to_string()} else {"".to_string()});
    itunes_extension.set_explicit( if metadata.explicit {"Yes".to_string()} else {"No".to_string()});

    let category = rss::extension::itunes::ITunesCategoryBuilder::default()
        .text(metadata.category)
        .build()
        .expect("should be a valid itunes category");
    itunes_extension.set_categories(vec!(category));

    let mut owner = ITunesOwner::default();
    owner.set_name(format!("{}", metadata.author));
    owner.set_email(format!("{}", metadata.email));
    itunes_extension.set_owner(owner);

    let mut namespaces: HashMap<String, String> = HashMap::new();
    namespaces.insert("itunes".to_string(), "http://www.itunes.com/dtds/podcast-1.0.dtd".to_string());
    
    rss::ChannelBuilder::default()
        .title(metadata.title)
        .description(metadata.description)
        .link(get_data_link(&format!("{}.xml", metadata.namespace), false))
        .itunes_ext(itunes_extension)
        .namespaces(namespaces)
        .language(metadata.language)
        .build()
        .unwrap()
}

fn get_cast_data() -> Vec<Cast> {

    let file = fs::File::open("./data/casts.json")
        .expect("casts file should open read only");

    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("casts file should be proper JSON");
    
    json.as_array()
        .expect("casts file should contain JSON array")
        .iter()
        .map(|data| {
            let cast: Cast = serde_json::from_value(data.clone())
                .expect("cast item should be in correct format");
            cast
        })
        .collect()
}

fn get_metadata() -> Metadata{
    let file = fs::File::open("./data/feed.json")
        .expect("file should open read only");
    
    let meta: Metadata = serde_json::from_reader(file)
        .expect("Could not parse metadata");
        
    meta
}