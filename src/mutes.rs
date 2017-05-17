use ::*;

pub fn get(c: &Config, max_id: Option<ID>, since_id: Option<ID>,
    limit: Option<u8>) -> APIResult<Vec<Account>> {
    let query_string = query_string!("max_id", max_id; "since_id", since_id; "limit", limit);
    let url = format!("https://{}/api/v1/mutes?{}", c.server, query_string);
    let mut response = c.get(url.as_ref())?;
    Ok(Account::to_array(&mut response))
}
