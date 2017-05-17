use ::*;
use std::fmt;

pub fn get(c: &Config) -> APIResult<Vec<Report>> {
    let url = format!("https://{}/api/v1/reports", c.server);
    let mut response = c.get(url.as_ref())?;
    Ok(Report::to_array(&mut response))
}

pub fn post(c: &Config, account_id: ID, status_ids: &[u32], comment: String) -> APIResult<Report> {
    // TODO refactor
    let a: Vec<String> = status_ids.into_iter().map(|x| x.to_string()).collect();
    let b = a.join(",");
    let ids = format!("[{}]", b);
    let form_body = form_body!("account_id", Some(account_id); "status_ids", Some(ids);
        "comment", Some(comment));
    let url = format!("https://{}/api/v1/reports", c.server);
    let mut response = c.post(url.as_ref(), &form_body)?;
    Ok(Report::from(&mut response))
}
