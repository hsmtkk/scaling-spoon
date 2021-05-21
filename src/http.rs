use std::collections::HashMap;
use reqwest;

const ENPHOTO_URL: &str = "https://en-photo.net";
const LOGIN_URL: &str = "https://en-photo.net/login";

pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Client {
        let client = reqwest::blocking::Client::builder().cookie_store(true).build().unwrap();
        return Client {client:client};
    }

    pub fn get_login(&self) -> String {
        let resp = self.client.get(LOGIN_URL).send().unwrap();
        let stat = resp.status();
        if !stat.is_success(){
            println!("HTTP GET failed {}", stat);
        }
        return resp.text().unwrap();
    }

    pub fn post_login(&self, username:&str, password:&str, token:&str) {
        let mut params = HashMap::new();
        params.insert("_token", token);
        params.insert("email", username);
        params.insert("password", password);
        let resp = self.client.post(LOGIN_URL).form(&params).send().unwrap();
        let stat  = resp.status();
        if !stat.is_success(){
            println!("HTTP POST failed {}", stat);
        }
    }
}

