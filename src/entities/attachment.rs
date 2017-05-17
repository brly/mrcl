extern crate json;

use json::JsonValue;
use ::ID;
use ::APIResult;

#[derive(Debug)]
pub struct Attachment {
    pub id: ID,
    pub type_: String,
    pub url: String,
    pub remote_url: Option<String>,
    pub preview_url: String,
    pub text_url: Option<String>,
}

impl Attachment {
    pub fn from(json: &mut JsonValue) -> Attachment {
        let id = json["id"].as_u32().unwrap_or(0);
        let type_ = json["type"].take_string().unwrap_or("".into());
        let url = json["url"].take_string().unwrap_or("".into());
        let remote_url = json["remote_url"].take_string();
        let preview_url = json["preview_url"].take_string().unwrap_or("".into());
        let text_url = json["text_url"].take_string();

        Attachment {
            id,
            type_,
            url,
            remote_url,
            preview_url,
            text_url,
        }
    }

    pub fn to_array(json: &mut JsonValue) -> Vec<Attachment> {
        let mut t = Vec::new();
        let n = json.len();
        for i in 0..n {
            let attachment = Attachment::from(&mut json[i]);
            t.push(attachment);
        }
        t
    }
}
