use regex::Regex;

#[derive(Debug)]
pub struct Channel {
    id: String,
}

pub fn id_from_url(url: &str) -> &str {
    Regex::new(r"\?v=(.*)")
        .unwrap()
        .captures(url)
        .expect("FeedItem url should match ?v=<id> pattern")
        .get(1)
        .expect("FeedItem url should match ?v=<id> pattern")
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