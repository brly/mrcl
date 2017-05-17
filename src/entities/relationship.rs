extern crate json;

use json::JsonValue;

#[derive(Debug)]
pub struct Relationship {
    id: u32,
    following: bool,
    followed_by: bool,
    blocking: bool,
    muting: bool,
    requested: bool,
}

impl Relationship {
    pub fn from(json: &mut JsonValue) -> Relationship {
        let id = json["id"].as_u32().unwrap_or(0);
        let following = json["following"].as_bool().unwrap_or(false);
        let followed_by = json["followed_by"].as_bool().unwrap_or(false);
        let blocking = json["blocking"].as_bool().unwrap_or(false);
        let muting = json["muting"].as_bool().unwrap_or(false);
        let requested = json["requested"].as_bool().unwrap_or(false);
        Relationship {
            id: id,
            following: following,
            followed_by: followed_by,
            blocking: blocking,
            muting: muting,
            requested: requested,
        }
    }

    pub fn to_array(json: &mut JsonValue) -> Vec<Relationship> {
        let mut t = Vec::new();
        let n = json.len();
        for i in 0..n {
            let relationship = Relationship::from(&mut json[i]);
            t.push(relationship);
        }
        t
    }
}
