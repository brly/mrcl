extern crate json;

use ::*;
use json::JsonValue;

#[derive(Debug)]
pub struct Notification {
    id: ID,
    type_: String,
    created_at: String,
    account: Account,
    status: Option<Status>,
}

impl Notification {
    pub fn from(json: &mut JsonValue) -> Notification {
        let id = json["id"].as_u32().unwrap_or(0);
        let type_ = json["type"].take_string().unwrap_or("".into());
        let created_at = json["created_at"].take_string().unwrap_or("".into());
        let account = Account::from(&mut json["account"]);
        let status = Some(Status::from(&mut json["status"]));
        Notification {
            id,
            type_,
            created_at,
            account,
            status,
        }
    }

    pub fn to_array(json: &mut JsonValue) -> Vec<Notification> {
        let mut t = Vec::new();
        let n = json.len();
        for i in 0..n {
            let notification = Notification::from(&mut json[i]);
            t.push(notification);
        }
        t
    }
}
