extern crate json;

use json::JsonValue;

use ::ID;

#[derive(Debug)]
pub struct Mention {
    pub url: String,
    pub username: String,
    pub acct: String,
    pub id: ID,
}

impl Mention {
    pub fn from(json: &mut JsonValue) -> Mention {
        let url = json["url"].take_string().unwrap_or("".into());
        let username = json["username"].take_string().unwrap_or("".into());
        let acct = json["acct"].take_string().unwrap_or("".into());
        let id = json["id"].as_u32().unwrap_or(0);

        Mention {
            url,
            username,
            acct,
            id,
        }
    }

    pub fn to_array(json: &mut JsonValue) -> Vec<Mention> {
        let mut t = Vec::new();
        let n = json.len();
        for i in 0..n {
            let mention = Mention::from(&mut json[i]);
            t.push(mention);
        }
        t
    }
}
