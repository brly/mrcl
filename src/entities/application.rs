extern crate json;

use json::JsonValue;

#[derive(Debug)]
pub struct Application {
    name: String,
    website: Option<String>,
}

impl Application {
    pub fn from(json: &mut JsonValue) -> Application {
        let name = json.take_string().unwrap_or("".into());
        let website = json.take_string();

        Application {
            name,
            website,
        }
    }
}
