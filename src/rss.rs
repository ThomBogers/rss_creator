use crate::cast::{CastItem, Cast};

use rss::extension::itunes::{ITunesChannelExtension, ITunesOwner};
use rss::ChannelBuilder;

use std::{ collections::HashMap, fs};

pub struct Rss {
    cast: Option<Cast>,
    channel: rss::Channel,
    pub settings: RssSettings,
}

#[derive(Debug)]
pub struct RssSettings {
    pub author: String,
    pub namespace: String,
    pub category: String,
    pub email: String,
    pub title: String,
    pub description: String,
    pub language: String,
    pub url: String,
    pub api_key: String,
    pub hide_from_store: bool,
    pub explicit: bool,
    pub data_dir: String,
}

fn get_data_link(filename: &str, settings: &RssSettings, namespaced: bool) -> String{
    let url = if namespaced {
        format!("{}/{}", settings.url, settings.namespace)
    } else {
        settings.url.to_string()
    };

    let api_key = &settings.api_key;
    if api_key.len() > 0 {
        format!("{}/{}?auth={}", url, filename, api_key)
    } else {
        format!("{}/{}", url, filename)
    }
}

fn item_from_cast(settings: &RssSettings, cast: &CastItem) -> rss::Item {
    println!("\tget feed item for cast: {:?}", cast);

    let file_meta = fs::metadata(format!("{}/{}", settings.data_dir, cast.filename))
        .expect(&format!("Could not open file {}/{}", settings.data_dir, cast.filename));

    let file_size = file_meta.len();

    let enclosure = rss::EnclosureBuilder::default()
        .url(get_data_link(&cast.filename, settings, true))
        .length(format!("{}", file_size))
        .mime_type("audio/mpeg")
        .build()
        .expect(&format!("Could not build rss enclosure {:?}", cast));

    rss::ItemBuilder::default()
        .link(get_data_link(&cast.filename, settings, true))
        .title(cast.title.clone())
        .author(cast.author.clone())
        .pub_date(cast.created_at.clone())
        .enclosure(enclosure)
        .itunes_ext(rss::extension::itunes::ITunesItemExtension::default())
        .dublin_core_ext(rss::extension::dublincore::DublinCoreExtension::default())
        .build()
        .expect(&format!("Could not build rss item {:?}", cast))
}

impl Rss {
    pub fn default(settings: RssSettings) -> Rss {
        let mut itunes_extension = ITunesChannelExtension::default();
        itunes_extension.set_author(format!("{}", &settings.author));
        itunes_extension.set_image(get_data_link(&format!("{}.png", &settings.namespace), &settings, false));
        itunes_extension.set_block( if settings.hide_from_store {"Yes".to_string()} else {"".to_string()});
        itunes_extension.set_explicit( if settings.explicit {"Yes".to_string()} else {"No".to_string()});

        let category = rss::extension::itunes::ITunesCategoryBuilder::default()
            .text(&settings.category)
            .build()
            .expect("should be a valid itunes category");
        itunes_extension.set_categories(vec!(category));

        let mut owner = ITunesOwner::default();
        owner.set_name(format!("{}", &settings.author));
        owner.set_email(format!("{}", &settings.email));
        itunes_extension.set_owner(owner);

        let mut namespaces: HashMap<String, String> = HashMap::new();
        namespaces.insert("itunes".to_string(), "http://www.itunes.com/dtds/podcast-1.0.dtd".to_string());
        
        let channel = ChannelBuilder::default()
            .title(&settings.title)
            .description(&settings.description)
            .link(get_data_link(&format!("{}.xml", &settings.namespace), &settings, false))
            .itunes_ext(itunes_extension)
            .namespaces(namespaces)
            .language(format!("{}", &settings.language))
            .build()
            .expect(&format!("Could not build rss channel {:?}", settings));
            
        Rss {settings, channel, cast: None}
    }

    pub fn set_cast(mut self, cast: Cast) -> Rss {
        self.cast = Some(cast);
        self
    }

    pub fn build(mut self) -> Rss {
        let res: Option<Vec<rss::Item>> = self.cast
            .as_ref()
            .and_then(|chan| {
                let items = chan.items()
                    .iter()
                    .map(|item| { return item_from_cast(&self.settings, item)})
                    .collect();
                Some(items)
            });

        if let Some(rss_items) = res {
            self.channel.set_items(rss_items);
        };

        self
    }

    pub fn to_string(&self) -> String {
        self.channel.to_string()
    }
}