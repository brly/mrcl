use ::*;

pub fn get(c: &Config, max_id: Option<ID>, since_id: Option<ID>, limit: Option<u8>) ->
    APIResult<Vec<Notification>> {
    let query_string = query_string!("max_id", max_id; "since_id", since_id; "limit", limit);
    let url = format!("https://{}/api/v1/notifications?{}", c.server, query_string);
    let mut response = c.get(url.as_ref())?;
    Ok(Notification::to_array(&mut response))
}

pub fn get_id(c: &Config, id: ID) -> APIResult<Notification> {
    let url = format!("https://{}/api/v1/notifications/{}", c.server, id);
    let mut response = c.get(url.as_ref())?;
    Ok(Notification::from(&mut response))
}

pub fn post(c: &Config) -> APIResult<bool> {
    let url = format!("https://{}/api/v1/notifications/clear", c.server);
    let response = c.get(url.as_ref())?;
    Ok(true)
}
