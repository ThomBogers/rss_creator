use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct FeedItem {
    pub id: String,
    pub author: String,
    pub title: String,
    pub link: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Cast {
    pub id: String,
    pub author: String,
    pub title: String,
    pub filename: String,
    pub created_at: String
}