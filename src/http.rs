use std::collections::HashMap;
use std::fs;
use url::Url;

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
        return self.http_get(LOGIN_URL);
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

    pub fn get_album(&self, album_url:&str) -> String {
        return self.http_get(album_url);
    }

    pub fn get_data_src(&self, data_srcs:&Vec<String>) -> Vec<String> {
        let mut urls : Vec<String> = Vec::new();
        for d in data_srcs {
            let url = ENPHOTO_URL.to_string() + &d;
            let content = self.http_get(&url);
            urls.push(content);
        }
        return urls;
    }

    pub fn get_thumbnail(&self, thumnail_urls:&Vec<String>){
        for t in thumnail_urls {
            let parsed = Url::parse(t).unwrap();
            let url_path = parsed.path();
            let elems: Vec<&str> = url_path.split('/').collect();
            let name = elems[elems.len()-1];
            let path = "photo/".to_string() + name;
            let resp = self.client.get(t).send().unwrap();
            let stat = resp.status();
            if !stat.is_success(){
                println!("HTTP GET failed {}", stat)
            }
            let content = resp.bytes().unwrap();
            fs::write(path, content).unwrap();
        }
    }

    fn http_get(&self, url:&str) -> String {
        let resp = self.client.get(url).send().unwrap();
        let stat = resp.status();
        if !stat.is_success(){
            println!("HTTP GET failed {}", stat);
        }
        return resp.text().unwrap();
    }
}
