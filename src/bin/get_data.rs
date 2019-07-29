use std::fs;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

use std::str::FromStr;

use atom_syndication::Feed;

use serde::Serialize;
use serde::Deserialize;
use serde_json;

use reqwest;

use std::process::Command;

use rss_creator::{FeedItem, Cast ,Options};
use structopt::StructOpt;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Metadata {
    channel_id: String,
    limit: usize
}

fn main() {
    println!("Starting data retrieval");
    let metadata = get_metadata();

    let url = get_feed_url();
    let feed = get_feed_data(&url);
    let mut casts = get_cast_data();

    let feed: Vec<FeedItem> = feed
        .into_iter()
        .take(metadata.limit)
        .filter(|item| {
            let res = casts
                .iter()
                .find(|cast| {item.id == cast.id});
                
            if let None = res {true} else {false}
        })
        .collect();

    feed
        .iter()
        .for_each(|item| {
            get_feed_audio(item)
                .expect("Should be able to download the file");
            casts.push(feed_item_to_cast(item));
        });

    let json = serde_json::to_string_pretty(&casts)
        .unwrap();
    write_casts(&json);

    println!("Done");
}

fn write_casts(casts: &str) {
    let options = Options::from_args();
    let mut f = File::create(format!("{}/casts.json", options.config_dir))
        .expect("Could not create casts.json");

    write!(f, "{}", casts).unwrap();
    f.sync_all().unwrap();
}

fn feed_item_to_cast(item: &FeedItem) -> Cast {
    Cast{
        author: item.author.to_string(), 
        created_at: item.created_at.to_string(), 
        title: item.title.to_string(), 
        filename: format!("{}.m4a", item.id),
        id: item.id.to_string()
    }
}

fn get_feed_audio(item: &FeedItem) -> Result<(), i32> {
    println!("Downloading id: {}", item.id);
    let options = Options::from_args();

    let output = Command::new("youtube-dl")
        .arg("--extract-audio")
        .arg("--audio-format")
        .arg("m4a")
        .arg("--audio-quality")
        .arg("9")
        .arg("--output")
        .arg(format!("{}/{}.%(ext)s",options.data_dir, item.id))
        .arg(&item.link)
        .output()
        .expect("Failed to download the file");

    // let output = Command::new("echo")
    //     .arg(&item.id)
    //     .output()
    //     .expect("Failed echo");

    match output.status.code() {
        Some(0) => Ok(()),
        Some(n) => Err(n),
        None => Err(-1)
    }
}

fn get_cast_data() -> Vec<Cast> {
    let options = Options::from_args();
    let file = match fs::File::open(format!("{}/casts.json", options.config_dir)) {
        Ok(value) => value,
        Err(_) => return vec!()
    };

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

fn get_feed_data(url: &str) -> Vec<FeedItem> {
    let data = reqwest::get(url)
        .unwrap()
        .text()
        .expect("The feed for the channel should be possible to download");

    let feed = Feed::from_str(&data)
        .expect("The feed should be parsable as a atom feed");

    feed
        .entries()
        .iter()
        .map(|item| {
            let title = item
                .title()
                .to_string();

            let author = item
                .authors()[0]
                .name()
                .to_string();
            
            let link = item
                .links()[0]
                .href()
                .to_string();
            
            let created_at = match item.published() {
                Some(date) => date,
                None => ""
            }.to_string();

            let id = get_id_from_url(&link)
                .to_string();

            FeedItem{title, author, link, created_at, id}
        })
        .collect()
}

fn get_id_from_url(url: &str) -> &str {
    Regex::new(r"\?v=(.*)")
        .unwrap()
        .captures(url)
        .expect("FeedItem url should match ?v=<id> pattern")
        .get(1)
        .expect("FeedItem url should match ?v=<id> pattern")
        .as_str()
}

fn get_feed_url() -> std::string::String {
    let metadata = get_metadata();
    format!("https://www.youtube.com/feeds/videos.xml?channel_id={}", metadata.channel_id)
}

fn get_metadata() -> Metadata{
    let options = Options::from_args();

    let file = fs::File::open(format!("{}/channel.json", options.config_dir))
        .expect("file should open read only");
    
    let meta: Metadata = serde_json::from_reader(file)
        .expect("Could not parse metadata");
        
    meta
}