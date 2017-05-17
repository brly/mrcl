extern crate json;

use json::JsonValue;

#[derive(Debug)]
pub struct Account {
    pub id: u32,
    pub username: String,
    pub acct: String,
    pub display_name: String,
    pub locked: bool,
    pub created_at: String,
    pub followers_count: u32,
    pub following_count: u32,
    pub statuses_count: u32,
    pub note: String,
    pub url: String,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
}

impl Account {
    pub fn new() -> Account {
        Account {
            id: 0,
            username: String::new(),
            acct: String::new(),
            display_name: String::new(),
            locked: false,
            created_at: String::new(),
            followers_count: 0,
            following_count: 0,
            statuses_count: 0,
            note: String::new(),
            url: String::new(),
            avatar: String::new(),
            avatar_static: String::new(),
            header: String::new(),
            header_static: String::new(),
        }
    }

    // TODO error handling
    pub fn from(json: &mut json::JsonValue) -> Account {
        let id = json["id"].as_u32().unwrap_or(0);
        let username = json["username"].take_string().unwrap_or("".into());
        let acct = json["acct"].take_string().unwrap_or("".into());
        let display_name = json["display_name"].take_string().unwrap_or("".into());
        let locked = json["locked"].as_bool().unwrap_or(false);
        let created_at = json["created_at"].take_string().unwrap_or("".into());
        let followers_count = json["followers_count"].as_u32().unwrap_or(0);
        let following_count = json["following_count"].as_u32().unwrap_or(0);
        let statuses_count = json["statuses_count"].as_u32().unwrap_or(0);
        let note = json["note"].take_string().unwrap_or("".into());
        let url = json["url"].take_string().unwrap_or("".into());
        let avatar = json["avatar"].take_string().unwrap_or("".into());
        let avatar_static = json["avatar_static"].take_string().unwrap_or("".into());
        let header = json["header"].take_string().unwrap_or("".into());
        let header_static = json["header_static"].take_string().unwrap_or("".into());
        Account {
            id: id,
            username: username,
            acct: acct,
            display_name: display_name,
            locked: locked,
            created_at: created_at,
            followers_count: followers_count,
            following_count: following_count,
            statuses_count: statuses_count,
            note: note,
            url: url,
            avatar: avatar,
            avatar_static: avatar_static,
            header: header,
            header_static: header_static,
        }
    }

    pub fn to_array(json: &mut JsonValue) -> Vec<Account> {
        let mut t = Vec::new();
        let n = json.len();
        for i in 0..n {
            let account = Account::from(&mut json[i]);
            t.push(account);
        }
        t
    }
}
