use ::*;

// TODO validate limit
pub fn get_favourited_by(c: &Config, id: u32, max_id: Option<u32>,
    since_id: Option<u32>, limit: Option<u32>) -> APIResult<Vec<Account>> {
    let query_string = query_string!("max_id", max_id; "since_id", since_id; "limit", limit);
    let url = format!("https://{}/api/v1/statuses/{}/favourited_by?{}", c.server, id, query_string);
    let mut response = c.get(&url)?;
    Ok(Account::to_array(&mut response))
}

pub fn patch_update_credentials(c: &Config, display_name: Option<String>,
    note: Option<String>, avatar: Option<String>, header: Option<String>) -> APIResult<Account> {
    let url = format!("https://{}/api/v1/accounts/update_credentials", c.server);
    let body = form_body!("display_name", display_name; "note", note; "avatar", avatar;
        "header", header);
    let mut response = c.patch(&url, &body)?;
    Ok(Account::from(&mut response))
}

pub fn get_followers(c: &Config, id: u32, max_id: Option<u32>, since_id: Option<u32>,
    limit: Option<u32>) -> APIResult<Vec<Account>> {
    let query_string = query_string!("max_id", max_id; "since_id", since_id; "limit", limit);
    let url = format!("https://{}/api/v1/accounts/{}/followers?{}", c.server, id, query_string);
    let mut response = c.get(url.as_ref())?;
    Ok(Account::to_array(&mut response))
}

pub fn get_following(c: &Config, id: u32, max_id: Option<u32>, since_id: Option<u32>,
    limit: Option<u32>) -> APIResult<Vec<Account>> {
    let query_string = query_string!("max_id", max_id; "since_id", since_id; "limit", limit);
    let url = format!("https://{}/api/v1/accounts/{}/following?{}", c.server, id, query_string);
    let mut response = c.get(url.as_ref())?;
    Ok(Account::to_array(&mut response))
}

pub fn get_statuses(c: &Config, id: u32, only_media: Option<bool>, exclude_replies: Option<bool>,
    max_id: Option<u32>, since_id: Option<u32>, limit: Option<u32>) -> APIResult<Vec<Status>> {
    let query_string = query_string!("only_media", only_media; "exclude_replies", exclude_replies;
        "max_id", max_id; "since_id", since_id; "limit", limit);
    let url = format!("https://{}/api/v1/accounts/{}/statuses?{}", c.server, id, query_string);
    let mut response = c.get(url.as_ref())?;
    Ok(Status::to_array(&mut response))
}

pub fn post_follow(c: &Config, id: u32) -> APIResult<Relationship> {
    let url = format!("https://{}/api/v1/accounts/{}/follow", c.server, id);
    let mut response = c.post(url.as_ref(), "")?;
    Ok(Relationship::from(&mut response))
}

pub fn post_unfollow(c: &Config, id: u32) -> APIResult<Relationship> {
    let url = format!("https://{}/api/v1/accounts/{}/unfollow", c.server, id);
    let mut response = c.post(url.as_ref(), "")?;
    Ok(Relationship::from(&mut response))
}

pub fn post_block(c: &Config, id: u32) -> APIResult<Relationship> {
    let url = format!("https://{}/api/v1/accounts/{}/block", c.server, id);
    let mut response = c.post(url.as_ref(), "")?;
    Ok(Relationship::from(&mut response))
}

pub fn post_unblock(c: &Config, id: u32) -> APIResult<Relationship> {
    let url = format!("https://{}/api/v1/accounts/{}/unblock", c.server, id);
    let mut response = c.post(url.as_ref(), "")?;
    Ok(Relationship::from(&mut response))
}

pub fn post_mute(c: &Config, id: u32) -> APIResult<Relationship> {
    let url = format!("https://{}/api/v1/accounts/{}/mute", c.server, id);
    let mut response = c.post(url.as_ref(), "")?;
    Ok(Relationship::from(&mut response))
}

pub fn post_unmute(c: &Config, id: u32) -> APIResult<Relationship> {
    let url = format!("https://{}/api/v1/accounts/{}/unmute", c.server, id);
    let mut response = c.post(url.as_ref(), "")?;
    Ok(Relationship::from(&mut response))
}

pub fn get_relationships(c: &Config, ids: &[u32]) -> APIResult<Vec<Relationship>> {
    // TODO refactor
    let a: Vec<String> = ids.into_iter().map(|x| x.to_string()).collect();
    let b = a.join(",");
    let query_string = format!("[{}]", b);
    let url = format!("https://{}/api/v1/accounts/relationships?{}", c.server, query_string);
    let mut response = c.get(url.as_ref())?;
    Ok(Relationship::to_array(&mut response))
}

pub fn get_search(c: &Config, q: &str, limit: Option<u32>) -> APIResult<Vec<Account>> {
    let query_string = query_string!("q", Some(q); "limit", limit);
    let url = format!("https://{}/api/v1/accounts/search?{}", c.server, query_string);
    let mut response = c.get(url.as_ref())?;
    Ok(Account::to_array(&mut response))
}

pub fn get(c: &Config, id: u32) -> APIResult<Account> {
    let url = format!("https://{}/api/v1/accounts/{}", c.server, id);
    let mut response = c.get(url.as_str())?;
    Ok(Account::from(&mut response))
}

pub fn get_verify_credentials(c: &Config) -> APIResult<Account> {
    let url = format!("https://{}/api/v1/accounts/verify_credentials", c.server);
    let mut response = c.get(url.as_str())?;
    Ok(Account::from(&mut response))
}
