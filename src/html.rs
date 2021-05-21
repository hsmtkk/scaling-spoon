use scraper::Html;
use scraper::Selector;

pub fn get_token(html: &str) -> Result<String, &str> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("input").unwrap();
    for element in document.select(&selector){
        if let Some(name) = element.value().attr("name"){
            if name == "_token" {
                if let Some(token) = element.value().attr("value"){
                    return Ok(token.to_string());
                }                
            }
        }
    }
    return Err("failed to find token");
}

#[test]
pub fn test_get_token(){
    let html = fs::read_to_string("src/login.html").unwrap();
    assert_eq!("WDSoa4MkKZG6G7JM2svtVnyyvSK2CojeewtY0gFt", get_token(&html).unwrap());
}