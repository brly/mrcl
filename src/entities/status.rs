extern crate json;

use json::JsonValue;
use ::ID;
use ::entities::Account;
use ::entities::Attachment;
use ::entities::Mention;
use ::entities::Tag;
use ::entities::Application;

#[derive(Debug)]
pub struct Status {
    pub id: ID,
    pub uri: String,
    pub url: String,
    pub account: Account,
    pub in_reply_to_id: Option<u32>,
    pub in_reply_to_account_id: Option<u32>,
    pub reblog: Option<Box<Status>>,
    pub content: String,
    pub created_at: String,
    pub reblogs_count: u32,
    pub favourites_count: u32,
    pub reblogged: bool,
    pub favourited: bool,
    pub sensitive: bool,
    pub spoiler_text: String,
    pub visibility: String,
    pub media_attachments: Vec<Attachment>,
    pub mentions: Vec<Mention>,
    pub tags: Vec<Tag>,
    pub application: Option<Application>,
}

impl Status {
    // TODO error handling
    pub fn from(json: &mut JsonValue) -> Status {
        let id = json["id"].as_u32().unwrap_or(0);
        let uri = json["uri"].take_string().unwrap_or(String::from(""));
        let url = json["url"].take_string().unwrap_or(String::from(""));
        let account = Account::from(&mut json["account"]);
        let in_reply_to_id = json["in_reply_to_id"].as_u32();
        let in_reply_to_account_id = json["in_reply_to_account_id"].as_u32();
        let reblog: Option<Box<Status>>;
        if json["reblog"].is_null() {
            reblog = None;
        } else {
            let reblog_status = Status::from(&mut json["reblog"]);
            reblog = Some(Box::new(reblog_status));
        }
        let content = json["content"].take_string().unwrap_or(String::from(""));
        let created_at = json["created_at"].take_string().unwrap_or(String::from(""));
        let reblogs_count = json["reblogs_count"].as_u32().unwrap_or(0);
        let favourites_count = json["favourites_count"].as_u32().unwrap_or(0);
        let reblogged = json["reblogged"].as_bool().unwrap_or(false);
        let favourited = json["favourited"].as_bool().unwrap_or(false);
        let sensitive = json["sensitive"].as_bool().unwrap_or(false);
        let spoiler_text = json["spoiler_text"].take_string().unwrap_or(String::from(""));
        let visibility = json["visibility"].take_string().unwrap_or(String::from(""));
        let media_attachments = Attachment::to_array(&mut json["media_attachments"]);
        let mentions = Mention::to_array(&mut json["mentions"]);
        let tags = Tag::to_array(&mut json["tags"]);
        let application = Some(Application::from(&mut json["application"]));

        Status {
            id: id,
            uri: uri,
            url: url,
            account: account,
            in_reply_to_id: in_reply_to_id,
            in_reply_to_account_id: in_reply_to_account_id,
            reblog: reblog,
            content: content,
            created_at: created_at,
            reblogs_count: reblogs_count,
            favourites_count: favourites_count,
            reblogged: reblogged,
            favourited: favourited,
            sensitive: sensitive,
            spoiler_text: spoiler_text,
            visibility: visibility,
            media_attachments,
            mentions,
            tags,
            application,
        }
    }

    pub fn to_array(json: &mut JsonValue) -> Vec<Status> {
        let mut entries = Vec::new();
        let n = json.len();
        for i in 0..n {
            let ref mut entry = json[i];
            let toot = Status::from(entry);
            entries.push(toot);
        }
        entries
    }
}
