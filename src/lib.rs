use structopt::StructOpt;

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

pub struct FileNames {}

impl FileNames {
    pub fn casts() -> String {
        "casts.json".to_string()
    }
    pub fn channel() -> String {
        "channel.json".to_string()
    }
    pub fn feed() -> String {
        "feed.json".to_string()
    }
}


pub mod channel;
pub mod feed;
pub mod cast;
pub mod rss;