use std::env;

pub fn get_credential() -> (String, String) {
    let username = env::var("ENPHOTO_USERNAME").expect("environment variable ENPHOTO_USERNAME is not defined");
    let password = env::var("ENPHOTO_PASSWORD").expect("environment variable ENPHOTO_PASSWORD is not defined");
    return (username.to_string(), password.to_string());
}
