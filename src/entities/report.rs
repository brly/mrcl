extern crate json;

use ::*;

#[derive(Debug)]
pub struct Report {
    pub id: ID,
    pub action_taken: Option<String>,
}

impl Report {
    pub fn from(json: &mut JsonValue) -> Report {
        let id = json["id"].as_u32().unwrap_or(0);
        let action_taken = json["action_taken"].take_string();
        Report {
            id,
            action_taken,
        }
    }

    pub fn to_array(json: &mut JsonValue) -> Vec<Report> {
        let mut t = Vec::new();
        let n = json.len();
        for i in 0..n {
            let report = Report::from(&mut json[i]);
            t.push(report);
        }
        t
    }
}
