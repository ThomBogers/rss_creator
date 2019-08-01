use std::{fs, io::prelude::*};

use rss_creator::{cast::Cast, rss::{Rss, RssSettings}, Options, FileNames};

use serde_json;
use serde::{Serialize, Deserialize};
use structopt::StructOpt;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Metadata {
    url: String,
    namespace: String,
    category: String,
    language: String,
    title: String,
    description: String,
    author: String,
    email: String,
    api_key: String,
    explicit: bool,
    hide_from_store: bool,
}

fn main() {
    let options = Options::from_args();
    let metadata = get_metadata(&options.config_dir);

    println!("Starting rss builder!");

    let cast = Cast::from_file(&format!("{}/{}", options.config_dir, FileNames::casts()));

    let rss_settings = RssSettings {
        url: metadata.url,
        namespace: metadata.namespace,
        category: metadata.category,
        language: metadata.language,
        title: metadata.title,
        description: metadata.description,
        author: metadata.author,
        email: metadata.email,
        api_key: metadata.api_key,
        explicit: metadata.explicit,
        hide_from_store: metadata.hide_from_store,
        data_dir: options.data_dir
    };

    let xml = Rss::default(rss_settings)
        .set_cast(cast)
        .build()
        .to_string();

    write_rss(&xml, &options.feed_dir);
    println!("Done");
}

fn write_rss(rss: &str, feed_dir: &str) {
    
    let mut f = fs::File::create(format!("{}/{}", feed_dir, FileNames::rss()))
        .expect("Could not create rss files");

    write!(f, "{}", rss).unwrap();
    f.sync_all().unwrap();
}

fn get_metadata(config_dir: &str) -> Metadata{
    let file = fs::File::open(format!("{}/{}", config_dir, FileNames::feed()))
        .expect("file should open read only");
    
    let meta: Metadata = serde_json::from_reader(file)
        .expect("Could not parse metadata");
        
    meta
}