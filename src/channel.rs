use regex::Regex;

#[derive(Debug)]
pub struct Channel {
    id: String,
}

pub fn id_from_url(url: &str) -> &str {
    Regex::new(r"\?v=(.*)")
        .expect("Could not create regex for channel id")
        .captures(url)
        .expect("Could not match url to to pattern ?v=<id>")
        .get(1)
        .expect("Could not get id from the pattern match")
        .as_str()
}

impl Channel {
    pub fn from_string(id: &str) -> Channel {
        Channel {id: id.to_string()}
    }
    
    pub fn get_url(&self) -> String {
        const YOUTUBE_FEED_URL: &str = "https://www.youtube.com/feeds/videos.xml?channel_id=";
        format!("{}{}", YOUTUBE_FEED_URL, self.id)
    }
}