extern crate hyper;
extern crate json;
extern crate url;
extern crate hyper_native_tls;

use json::JsonValue;
use hyper::header::{Authorization, Bearer};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;

macro_rules! query_string {
    (
        $( $x:expr, $y:expr );*
    ) => {{
        let mut t = Vec::new();
        $(
            if let Some(s) = $y {
                t.push(format!("{}={}", $x, s));
            }
        )*
        t.join("&")
    }};
}

macro_rules! form_body {
    (
        $( $x:expr, $y:expr );*
    ) => {{
        let mut b = url::form_urlencoded::Serializer::new(String::new());
        $(
            if let Some(s) = $y {
                let kv = format!("{}", s);
                b.append_pair($x, &kv);
            }
        )*
        b.finish()
    }};
}

#[derive(Debug)]
pub enum APIError {
    HyperError(hyper::Error),
    JsonError(json::Error),
    NoAccessToken,
}

impl From<hyper::Error> for APIError {
    fn from(err: hyper::Error) -> APIError {
        APIError::HyperError(err)
    }
}

impl From<json::Error> for APIError {
    fn from(err: json::Error) -> APIError {
        APIError::JsonError(err)
    }
}

type APIResult<T> = std::result::Result<T, APIError>;
type ID = u32;

#[derive(Debug)]
pub struct Config {
    client: hyper::Client,
    access_token: Option<String>,
    server: String,
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
    scope: String,
}

impl Config {
    pub fn new(server: &str, client_id: &str, client_secret: &str, username: &str,
        password: &str, scope: &str) -> Config {
        let tls = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(tls);
        let client = hyper::Client::with_connector(connector);
        Config {
            client: client,
            access_token: None,
            server: server.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            username: username.into(),
            password: password.into(),
            scope: scope.into(),
        }
    }

    pub fn authenticate(&mut self) -> APIResult<bool> {
        let body = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("client_id", self.client_id.as_ref())
            .append_pair("client_secret", self.client_secret.as_ref())
            .append_pair("grant_type", "password")
            .append_pair("username", self.username.as_ref())
            .append_pair("password", self.password.as_ref())
            .append_pair("scope", self.scope.as_ref())
            .finish();
        let url = format!("https://{}/oauth/token", self.server);
        let mut buf = String::new();
        self.client.post(&url).body(&body).send()
            .and_then(|mut e|{
                e.read_to_string(&mut buf)?;
                Ok(e)
            })?;
        let mut parsed = json::parse(buf.as_str())?;
        self.access_token = parsed["access_token"].take_string();
        Ok(true)
    }

    pub fn get(&self, url: &str) -> APIResult<JsonValue> {
        let mut buf = String::new();
        let access_token = self.access_token.clone().ok_or(APIError::NoAccessToken)?;
        self.client.get(url)
            .header(Authorization(Bearer {token: access_token}))
            .send()
            .and_then(|mut e|{
                e.read_to_string(&mut buf)?;
                Ok(e)
            })?;
        json::parse(buf.as_ref()).map_err(|e| APIError::JsonError(e))
    }

    pub fn post(&self, url: &str, body: &str) -> APIResult<JsonValue> {
        let mut buf = String::new();
        let access_token = self.access_token.clone().ok_or(APIError::NoAccessToken)?;
        self.client.post(url)
            .header(Authorization(Bearer {token: access_token}))
            .body(body)
            .send()
            .and_then(|mut e| {
                e.read_to_string(&mut buf)?;
                Ok(e)
            })?;
        json::parse(buf.as_ref()).map_err(|e| APIError::JsonError(e))
    }

    pub fn patch(&self, url: &str, body: &str) -> APIResult<JsonValue> {
        let mut buf = String::new();
        let access_token = self.access_token.clone().ok_or(APIError::NoAccessToken)?;
        println!("patch-body:{:?}", body);
        self.client.patch(url)
            .header(Authorization(Bearer {token: access_token}))
            .body(body)
            .send()
            .and_then(|mut e|{
                e.read_to_string(&mut buf)?;
                Ok(e)
            })?;
        json::parse(buf.as_ref()).map_err(|e| APIError::JsonError(e))
    }


}

pub mod entities;
use entities::{Account, Status, Relationship, Instance, Notification, Report, Results};
use entities::{Context, Card};

// TODO refactor
fn parse_string_array(json: &mut JsonValue) -> Vec<String> {
    let mut t = Vec::new();
    let n = json.len();
    for i in 0..n {
        let s = json[i].take_string().unwrap_or("".into());
        t.push(s)
    }
    t
}

pub mod statuses;
pub mod accounts;
pub mod apps;
pub mod blocks;
pub mod favourites;
pub mod follow_requests;
pub mod follows;
pub mod instances;
pub mod mutes;
pub mod notifications;
pub mod reports;
pub mod search;
