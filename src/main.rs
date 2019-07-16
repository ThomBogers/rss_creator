use std::fs;
use std::fs::File;
use std::io::prelude::*;

use rss;
use rss::extension::itunes::ITunesChannelExtension;

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
    namespace: String,
    category: String,
    title: String,
    description: String,
    author: String,
    api_key: String,
    hide_from_store: bool,
}

fn main() {
    println!("Starting rss builder!");

    let casts = get_castdata();
    let feed_items: Vec<rss::Item> = casts
        .iter()
        .map(get_feed_item)
        .collect();

    let mut channel = get_channel();
    channel.set_items(feed_items);

    let xml = channel.to_string();
    write_feed(&xml);
    println!("Result:\n{}", xml);
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
    let metadata = get_metadata();

    let file_meta = fs::metadata(format!("./data/{}", cast.filename))
        .expect(&format!("Could not open file ./data/{}", cast.filename));

    let file_size = file_meta.len();

    let enclosure = rss::EnclosureBuilder::default()
        .url(get_data_link(&cast.filename, true))
        .length(format!("{}", file_size))
        .mime_type("m4a")
        .build()
        .unwrap();

    let mut category = rss::Category::default();
    category.set_name(metadata.category);

    let categories = vec!(category);

    rss::ItemBuilder::default()
        .link(get_data_link(&cast.filename, true))
        .title(cast.episodename.clone())
        .author(cast.author.clone())
        .pub_date(cast.created_at.clone())
        .enclosure(enclosure)
        .categories(categories)
        .build()
        .unwrap()
}

fn get_channel() -> rss::Channel {
    let metadata = get_metadata();

    let mut itunes_extension = ITunesChannelExtension::default();
    itunes_extension.set_author(metadata.author);
    itunes_extension.set_image(get_data_link(&format!("{}.png", metadata.namespace), false));
    itunes_extension.set_block( if metadata.hide_from_store {"Yes".to_string()} else {"".to_string()});

    rss::ChannelBuilder::default()
        .title(metadata.title)
        .description(metadata.description)
        .link(get_data_link(&format!("{}.xml", metadata.namespace), false))
        .itunes_ext(itunes_extension)
        .build()
        .unwrap()
}

fn get_castdata() -> Vec<Cast> {

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
    let file = fs::File::open("./data/meta.json")
        .expect("file should open read only");
    
    let meta: Metadata = serde_json::from_reader(file)
        .expect("Could not parse metadata");
        
    meta
}