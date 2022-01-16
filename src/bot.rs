use serde_json::{json, Value};
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use reqwest::Error;
use std::str::FromStr;

pub struct Bot{
    protocol: String,
    base_uri: String,
    token: String,
}

impl Bot{
    pub fn new(protocol: &str, base_uri: &str, token: &str) -> Bot{
        Self {
            protocol: protocol.to_string(),
            base_uri: base_uri.to_string(),
            token: token.to_string(),
        }
    }

    pub fn post_message(&self, channel_id: &str, message: &str, root_id: Option<&str>) -> Result<Response, Error>{
        let url = format!("{}://{}/api/v4/posts", self.protocol, self.base_uri);
        let body = if let Some(value) = root_id{
            json!({
                "channel_id": channel_id,
                "message": message,
                "root_id": value
            })
        }else{
            json!({
                "channel_id": channel_id,
                "message": message,
            })
        };
        self.post(&url, Some(body))
    }

    pub fn list_users(&self) ->Result<Response, Error>{
        let url = format!("{}://{}/api/v4/users", self.protocol, self.base_uri);
        self.get(&url)
    }

    pub fn list_channels(&self) ->Result<Response, Error>{
        let url = format!("{}://{}/api/v4/channels", self.protocol, self.base_uri);
        self.get(&url)
    }

    fn get(&self, url: &str)->Result<Response, Error>{
        println!("URL: {}", url);
        let mut header_map = HeaderMap::new();
        println!("{}", self.token);
        header_map.insert(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap());
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        client.get(url).send()
    }

    fn post(&self, url: &str, body: Option<Value>)->Result<Response, Error>{
        println!("URL: {}", url);
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.insert(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap());
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        match body{
            Some(value) => {
                let content = serde_json::to_string(&value).unwrap();
                println!("The content: {}", content);
                client.post(url).body(content).send()},
            None => client.post(url).send(),
        }
    }
}
