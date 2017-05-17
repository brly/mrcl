use ::*;

pub fn get(c: &Config) -> APIResult<Instance> {
    let url = format!("https://{}/api/v1/instance", c.server);
    let mut response = c.get(&url)?;
    Ok(Instance::from(&mut response))
}
