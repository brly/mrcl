use ::*;

pub fn get(c: &Config, q: String, resolve: bool) -> APIResult<Results> {
    //let form_data = form_data!("q", q; "resolve", resolve);
    let query_string = query_string!("q", Some(q); "resolve", Some(resolve));
    let url = format!("https://{}/api/v1/search?{}", c.server, query_string);
    let mut response = c.get(url.as_ref())?;
    Ok(Results::from(&mut response))
}
