use ::*;

pub fn get(c: &Config, max_id: Option<ID>, since_id: Option<ID>, limit: Option<u8>)
    -> APIResult<Vec<Account>> {
    let query_string = query_string!("max_id", max_id; "since_id", since_id; "limit", limit);
    let url = format!("https://{}/api/v1/follow_requests?{}", c.server, query_string);
    let mut response = c.get(url.as_ref())?;
    Ok(Account::to_array(&mut response))
}

pub fn post_authorize(c: &Config, id: ID) -> APIResult<bool> {
    let url = format!("https://{}/api/v1/follow_requests/{}/authorize", c.server, id);
    let mut response = c.post(url.as_ref(), "")?;
    Ok(true)
}

pub fn post_reject(c: &Config, id: ID) -> APIResult<bool> {
    let url = format!("https://{}/api/v1/follow_requests/{}/reject", c.server, id);
    let mut response = c.post(url.as_ref(), "")?;
    Ok(true)
}
