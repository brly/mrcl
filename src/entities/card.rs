extern crate json;

use json::JsonValue;

#[derive(Debug)]
pub struct Card {
    pub url: String,
    pub title: String,
    pub description: String,
    pub image: Option<String>,
}

impl Card {
    pub fn from(json: &mut JsonValue) -> Card {
        let url = json["url"].take_string().unwrap_or("".into());
        let title = json["title"].take_string().unwrap_or("".into());
        let description = json["description"].take_string().unwrap_or("".into());
        let image = json["image"].take_string();
        Card {
            url, title, description, image
        }
    }
}
