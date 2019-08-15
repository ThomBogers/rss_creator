use crate::channel::{Channel, id_from_url};

use serde::{Serialize, Deserialize};
use atom_syndication::Feed as AtomFeed;
use reqwest;
use std::{
    process::{Command, Stdio},
    str::FromStr,
    io::{BufRead,BufReader}
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
    /// succes=true if the file has been downloaded,
    /// success=false if it is not possible to download the file
    /// Error if an error occurred during the retrieval
    pub fn get_audio(&self, data_dir: &str) -> Result<bool, i32> {
        println!("Downloading id: {}", self.id);

        let mut output = Command::new("youtube-dl")
            .arg("--extract-audio")
            .arg("--audio-format")
            .arg("m4a")
            .arg("--audio-quality")
            .arg("9")
            .arg("--output")
            .arg(format!("{}/{}.%(ext)s",data_dir, self.id))
            .arg(&self.link)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to download the file");

        // let output = Command::new("echo")
        //     .arg(&item.id)
        //     .output()
        //     .expect("Failed echo");

        let mut audio_available = true;

        if let Some(ref mut stderr) = output.stderr {
            for line in BufReader::new(stderr).lines() {
                let line = line.unwrap();
                if line.contains("This live event will begin in") {
                    audio_available=false;
                }
                println!("[stderr] {}", line);
            }
        }        
        let status = output
            .wait()
            .expect("Command wasn't running");

        if !audio_available {
            return Ok(audio_available);
        }

        match status.code() {
            Some(0) => Ok(audio_available),
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