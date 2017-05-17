extern crate json;

use json::JsonValue;

#[derive(Debug)]
pub struct Instance {
    uri: String,
    title: String,
    description: String,
    email: String,
    version: Option<String>,
}

impl Instance {
    pub fn from(json: &mut JsonValue) -> Instance {
        let uri = json["uri"].take_string().unwrap_or("".into());
        let title = json["title"].take_string().unwrap_or("".into());
        let description = json["description"].take_string().unwrap_or("".into());
        let email = json["email"].take_string().unwrap_or("".into());
        let version = json["version"].take_string();
        Instance {
            uri: uri,
            title: title,
            description: description,
            email: email,
            version: version,
        }
    }
}
