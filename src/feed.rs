use crate::channel::{Channel, id_from_url};

use serde::{Serialize, Deserialize};
use atom_syndication::Feed as AtomFeed;
use reqwest;
use std::{
    process::Command,
    str::FromStr,
};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct FeedItem {
    pub id: String,
    pub author: String,
    pub title: String,
    pub link: String,
    pub created_at: String,
}

impl FeedItem {
    pub fn get_audio(&self, data_dir: &str) -> Result<(), i32> {
        println!("Downloading id: {}", self.id);

        let output = Command::new("youtube-dl")
            .arg("--extract-audio")
            .arg("--audio-format")
            .arg("m4a")
            .arg("--audio-quality")
            .arg("9")
            .arg("--output")
            .arg(format!("{}/{}.%(ext)s",data_dir, self.id))
            .arg(&self.link)
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
}

pub struct Feed {
    pub items: Vec<FeedItem>,
}

impl Feed {
    pub fn from_channel(channel: &Channel) -> Feed {
        let data = reqwest::get(&channel.get_url())
            .expect(&format!("Could not fetch url {:?}", channel))
            .text()
            .expect(&format!("Could not get text content from url {:?}", channel));

        let feed = AtomFeed::from_str(&data)
            .expect(&format!("Could not parse response as an atom feed {}", &data));

        let items = feed
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

                let id = id_from_url(&link)
                    .to_string();

                FeedItem{title, author, link, created_at, id}
            })
            .collect();
        
        Feed { items }
    }
}