use ::*;

pub fn post(c: &Config, uri: &str) -> APIResult<Account> {
    let url = format!("https://{}/api/v1/follows", c.server);
    let body = form_body!("uri", Some(uri));
    let mut response = c.post(url.as_ref(), body.as_ref())?;
    Ok(Account::from(&mut response))
}
