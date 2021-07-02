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
    let album_html = client.get_album(url);
    let data_srcs = html::get_data_src(&album_html);
    for d in &data_srcs {
        println!("{}", d);
    }
    let thumnail_urls = client.get_data_src(&data_srcs);
    for t in &thumnail_urls {
        println!("{}", t);
    }
    client.get_thumbnail(&thumnail_urls);
}

