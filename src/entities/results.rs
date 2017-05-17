extern crate json;

use ::*;
use json::JsonValue;

#[derive(Debug)]
pub struct Results {
    pub accounts: Vec<Account>,
    pub statuses: Vec<Status>,
    pub hashtags: Vec<String>,
}

impl Results {
    pub fn from(json: &mut JsonValue) -> Results {
        let accounts = Account::to_array(&mut json["accounts"]);
        let statuses = Status::to_array(&mut json["statuses"]);
        let hashtags = parse_string_array(&mut json["hashtags"]);
        Results {
            accounts, statuses, hashtags,
        }
    }
}
