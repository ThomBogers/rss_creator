use std::fs;

use rss_creator::{channel::Channel, feed::{Feed, FeedItem}, cast::{Cast, CastItem} ,Options, FileNames};

use serde_json;
use serde::{Serialize, Deserialize};
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
    let metadata = get_metadata(&options.config_dir);

    let channel = Channel::from_string(&metadata.channel_id);
    let feed    = Feed::from_channel(&channel);

    let mut cast = Cast::from_file(&format!("{}/{}", options.config_dir, FileNames::casts()));
    let mut casts = cast.items();

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
            let succes = item.get_audio(&options.data_dir)
                .expect("Should be able to download the file");
            
            if succes {
                casts.push(CastItem::from_feed_item(&item));
            }
        });

    cast.set_items(casts);
    cast.write(&format!("{}/{}", options.config_dir, FileNames::casts()));
    println!("Done");
}

fn get_metadata(config_dir: &str) -> Metadata{
    let file = fs::File::open(format!("{}/{}", config_dir, FileNames::channel()))
        .expect("file should open read only");
    
    let meta: Metadata = serde_json::from_reader(file)
        .expect("Could not parse metadata");
        
    meta
}