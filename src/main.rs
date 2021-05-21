use std::env;

mod cred;
mod html;
mod http;

fn main() {
    let (username, password) = cred::get_credential();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} url", args[0]);
        return;
    }
    let url = &args[1];
    println!("url: {}", url);
    let client = http::Client::new();
    let html = client.get_login();
    let token = html::get_token(&html).unwrap();
    println!("token: {}", token);
    client.post_login(&username, &password, &token);
}

