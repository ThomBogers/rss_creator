use std::{
    fs, 
    io::prelude::*, 
};

use serde::{Serialize, Deserialize};
use serde_json;

use rss_creator::{channel::Channel, feed::{Feed, FeedItem}, CastItem ,Options};
use structopt::StructOpt;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Metadata {
    channel_id: String,
    limit: usize
}

fn main() {
    println!("Starting data retrieval");
    let options = Options::from_args();
    let metadata = get_metadata();

    let channel = Channel::from_string(&metadata.channel_id);
    let feed     = Feed::from_channel(&channel);

    let mut casts = get_cast_data();

    let feed: Vec<FeedItem> = feed
        .items
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
            item.get_audio(&options.data_dir)
                .expect("Should be able to download the file");
            casts.push(CastItem::from_feed_item(item));
        });

    let json = serde_json::to_string_pretty(&casts)
        .unwrap();
    write_casts(&json);

    println!("Done");
}

fn write_casts(casts: &str) {
    let options = Options::from_args();
    let mut f = fs::File::create(format!("{}/casts.json", options.config_dir))
        .expect("Could not create casts.json");

    write!(f, "{}", casts).unwrap();
    f.sync_all().unwrap();
}

fn get_cast_data() -> Vec<CastItem> {
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
            let cast: CastItem = serde_json::from_value(data.clone())
                .expect("cast item should be in correct format");
            cast
        })
        .collect()
}

fn get_metadata() -> Metadata{
    let options = Options::from_args();

    let file = fs::File::open(format!("{}/channel.json", options.config_dir))
        .expect("file should open read only");
    
    let meta: Metadata = serde_json::from_reader(file)
        .expect("Could not parse metadata");
        
    meta
}