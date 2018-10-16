extern crate regex;
extern crate reqwest;

use self::regex::Regex;
use super::utils;

fn preprocess_body(body: String) -> String {
    let re = Regex::new(r" +").unwrap();
    let re2 = Regex::new(r"<!\[CDATA\[(.+?)\]\](.+?)>").unwrap();

    let mut tmp = body.replace("\n", "");
    tmp = re.replace_all(&tmp, " ").to_string();
    tmp = re2.replace_all(&tmp, r"$1").to_string();
    return tmp;
}

pub fn fetch(url: &str, fname: String) -> Result<(), reqwest::Error> {
    println!("Fetching {}...", url);

    let xml_tag_regex =
        |tag: String| -> Regex { Regex::new(&format!(r"<{}>(.+?)</{}>", tag, tag)).unwrap() };
    let mut body = reqwest::get(url)?.text()?;

    body = preprocess_body(body);

    for item in xml_tag_regex("item".to_string()).captures_iter(&body) {
        utils::write_item(item[1].to_string(), &fname);
    }
    Ok(())
}
