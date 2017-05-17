extern crate json;

use super::status::Status;
use json::JsonValue;

#[derive(Debug)]
pub struct Context {
    pub ancestors: Vec<Status>,
    pub descendants: Vec<Status>,
}

impl Context {
    pub fn from(json: &mut JsonValue) -> Context {
        let ancestors = Status::to_array(&mut json["ancestors"]);
        let descendants = Status::to_array(&mut json["descendants"]);
        Context {
            ancestors,
            descendants,
        }
    }
}
