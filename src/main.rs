use std::fs;

use rss;

use serde::Serialize;
use serde::Deserialize;
use serde_json;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Cast {
    filename: String,
    episodename: String,
    author: String,
    created_at: String
}

fn main() {
    println!("Start rss builder!");

    let file = fs::File::open("./data/casts.json")
        .expect("file should open read only");

    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");
    
    let data = json.as_array()
        .expect("file should contain JSON array");

    let mut channel = rss::ChannelBuilder::default()
        .title("Ksyos kennis sessies")
        .link("http://ksyos.nl/")
        .description("An RSS feed.")
        .build()
        .unwrap();
    
    let mut items = channel.items().to_vec();

    for index in 0..data.len() {
        let item = data.get(index).unwrap().clone();
        let cast: Cast = serde_json::from_value(item).unwrap();
        let feed_item = get_feed_item(cast);
        items.push(feed_item);
    }

    channel.set_items(items);
    let xml = channel.to_string();
    println!("Result:\n{}", xml);
}

fn get_feed_item(cast: Cast) -> rss::Item {
    println!("\tget feed item for cast: {:?}", cast);

    let enclosure = rss::EnclosureBuilder::default()
        .url(format!("http://ksyos.nl/casts/{}", cast.episodename))
        .length("123") //TODO: calculate this
        .mime_type("m4a")
        .build()
        .unwrap();

    let mut category = rss::Category::default();
    category.set_name("Business"); // TODO: add meta/feed.json file?

    let categories = vec!(category);

    rss::ItemBuilder::default()
        .link(format!("http://ksyos.nl/casts/{}", cast.episodename))
        .title(cast.episodename)
        .author(cast.author)
        .enclosure(enclosure)
        .categories(categories)
        .build()
        .unwrap()
}