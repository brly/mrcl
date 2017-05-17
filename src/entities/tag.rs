extern crate json;

use json::JsonValue;

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub url: String,
}

impl Tag {
    pub fn from(json: &mut JsonValue) -> Tag {
        let name = json["name"].take_string().unwrap_or("".into());
        let url = json["url"].take_string().unwrap_or("".into());

        Tag {
            name,
            url,
        }
    }

    pub fn to_array(json: &mut JsonValue) -> Vec<Tag> {
        let mut t = Vec::new();
        let n = json.len();
        for i in 0..n {
            let tag = Tag::from(&mut json[i]);
            t.push(tag);
        }
        t
    }
}
