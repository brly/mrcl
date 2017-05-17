use ::*;

pub fn post(c: &Config, client_name: &str, redirect_uris: &str, scopes: &str,
    website: Option<String>) -> APIResult<JsonValue> {
    // TODO make sturct
    let body = form_body!("client_name", Some(client_name); "redirect_uris", Some(redirect_uris);
        "scopes", Some(scopes); "website", website);
    let url = format!("https://{}/api/v1/apps", c.server);
    c.post(url.as_ref(), &body)
}
