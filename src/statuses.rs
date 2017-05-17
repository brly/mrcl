use entities::Account;
use ::*;

pub fn get(c: &Config, id: ID) -> APIResult<Status> {
    let url = format!("https://{}/api/v1/statuses/{}", c.server, id);
    let mut response = c.get(url.as_ref())?;
    Ok(Status::from(&mut response))
}

pub fn get_context(c: &Config, id: ID) -> APIResult<Context> {
    let url = format!("https://{}/api/v1/{}/context", c.server, id);
    let mut response = c.get(url.as_ref())?;
    Ok(Context::from(&mut response))
}

pub fn get_card(c: &Config, id: ID) -> APIResult<Card> {
    let url = format!("https://{}/api/v1/{}/card", c.server, id);
    let mut response = c.get(url.as_ref())?;
    Ok(Card::from(&mut response))
}

pub fn post(c: &Config, status: String, in_reply_to_id: Option<ID>,
    media_ids: Option<Vec<ID>>, sensitive: Option<bool>,
    spoiler_text: Option<String>, visibility: Option<String>) -> APIResult<Status> {
    let url = format!("https://{}/api/v1/statuses", c.server);
    // TODO refactor
    let a: Vec<String> = match media_ids {
        Some(m) => m.into_iter().map(|x| x.to_string()).collect(),
        None => Vec::new(),
    };
    let b = a.join(",");
    let ids = format!("[{}]", b);
    let body = form_body!("status", Some(status); "in_reply_to_id", in_reply_to_id;
        "spoiler_text", spoiler_text; "visibility", visibility; "sensitive", sensitive;
        "media_ids", Some(ids));
    let mut response = c.post(&url, &body)?;
    Ok(Status::from(&mut response))
}

pub fn post_reblog(c: &Config, id: u32) -> APIResult<Status> {
    let url = format!("https://{}/api/v1/statuses/{}/reblog", c.server, id);
    let mut response = c.post(&url, "")?;
    Ok(Status::from(&mut response))
}

pub fn post_unreblog(c: &Config, id: u32) -> APIResult<Status> {
    let url = format!("https://{}/api/v1/statuses/{}/unreblog", c.server, id);
    let mut response = c.post(&url, "")?;
    Ok(Status::from(&mut response))
}

pub fn post_favourite(c: &Config, id: u32) -> APIResult<Status> {
    let url = format!("https://{}/api/v1/statuses/{}/favourite", c.server, id);
    let mut response = c.post(&url, "")?;
    Ok(Status::from(&mut response))
}

pub fn post_unfavourite(c: &Config, id: u32) -> APIResult<Status> {
    let url = format!("https://{}/api/v1/statuses/{}/unfavourite", c.server, id);
    let mut response = c.post(&url, "")?;
    Ok(Status::from(&mut response))
}

pub fn get_timelines_home(c: &Config, local: Option<bool>, max_id: Option<u32>,
    since_id: Option<u32>, limit: Option<u8>) -> APIResult<Vec<entities::Status>> {
    let query_string = query_string!("local", local; "max_id", max_id;
        "since_id", since_id; "limit", limit);
    let url = format!("https://{}/api/v1/timelines/home?{}", c.server, query_string);
    let mut response = c.get(url.as_str())?;
    Ok(Status::to_array(&mut response))
}

pub fn get_timelines_public(c: &Config, local: Option<bool>, max_id: Option<u32>,
    since_id: Option<u32>, limit: Option<u8>) -> APIResult<Vec<entities::Status>> {
    let query_string = query_string!("local", local; "max_id", max_id;
        "since_id", since_id; "limit", limit);
    let url = format!("https://{}/api/v1/timelines/public?{}", c.server, query_string);
    let mut response = c.get(url.as_str())?;
    Ok(Status::to_array(&mut response))
}

pub fn get_timelines_tag(c: &Config, hashtag: String, local: Option<bool>, max_id: Option<u32>,
    since_id: Option<u32>, limit: Option<u8>) -> APIResult<Vec<Status>> {
    let query_string = query_string!("local", local; "max_id", max_id;
        "since_id", since_id; "limit", limit);
    let url = format!("https://{}/api/v1/timelines/tag/{}?{}", c.server, hashtag, query_string);
    let mut response = c.get(url.as_str())?;
    Ok(Status::to_array(&mut response))
}
