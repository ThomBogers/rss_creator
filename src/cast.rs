use crate::feed::FeedItem;

use serde::{Serialize, Deserialize};
use std::{
    fs, 
    io::prelude::*, 
};

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
#[derive(Debug)]
pub struct CastItem {
    pub id: String,
    pub author: String,
    pub title: String,
    pub filename: String,
    pub created_at: String
}

#[derive(Debug)]
impl CastItem {
    pub fn from_feed_item(item: &FeedItem) -> CastItem {
        CastItem{
                author: item.author.to_string(), 
                created_at: item.created_at.to_string(), 
                title: item.title.to_string(), 
                filename: format!("{}.m4a", item.id),
                id: item.id.to_string()
            }
    }
}

#[derive(Debug)]
pub struct Cast {
    items: Vec<CastItem>,
}

impl Cast {
    pub fn from_file(file: &str) -> Cast {
        let file = match fs::File::open(file) {
            Ok(value) => value,
            Err(_) => return Cast {items: vec!()}
        };

        let json: serde_json::Value = serde_json::from_reader(file)
            .expect("casts file should be proper JSON");
        
        let items = json.as_array()
            .expect("casts file should contain JSON array")
            .iter()
            .map(|data| {
                let cast: CastItem = serde_json::from_value(data.clone())
                    .expect("cast item should be in correct format");
                cast
            })
            .collect();
        
        Cast {items}
    }

    pub fn write(&self, path: &str) {
        let json = serde_json::to_string_pretty(&self.items)
            .expect(&format!("Could not serialize to json {:?}", &self));

        let mut f = fs::File::create(path)
            .expect(&format!("Could not create casts file {}", path));

        write!(f, "{}", json)
            .expect(&format!("Could not write to casts file {}", path));
        
        f.sync_all()
            .expect("Could not sync filesystem");
    }

    pub fn items(&self) -> Vec<CastItem> {
        self.items.to_vec()
    }

    pub fn set_items(&mut self, items: Vec<CastItem>) {
        self.items = items;
    }


}