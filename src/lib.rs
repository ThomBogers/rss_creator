use serde::{Serialize, Deserialize};

use structopt::StructOpt;
pub mod channel;
pub mod feed;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct CastItem {
    pub id: String,
    pub author: String,
    pub title: String,
    pub filename: String,
    pub created_at: String
}

impl CastItem {
    pub fn from_feed_item(item: &feed::FeedItem) -> CastItem {
        CastItem{
                author: item.author.to_string(), 
                created_at: item.created_at.to_string(), 
                title: item.title.to_string(), 
                filename: format!("{}.m4a", item.id),
                id: item.id.to_string()
            }
    }
}


#[derive(StructOpt, Debug)]
#[structopt(name = "rss_creator", about = "Create a podcast rss feed based on a youtube channe")]
pub struct Options {
    /// Location to read/write config files
    #[structopt(short = "c", long = "config", default_value = "./conf")]
    pub config_dir: String,

    /// Location to read/write data files
    #[structopt(short = "d", long = "data", default_value = "./data")]
    pub data_dir: String,

    /// Location to write the feed
    #[structopt(short = "f", long = "feed", default_value = "./")]
    pub feed_dir: String
}