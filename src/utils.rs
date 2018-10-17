extern crate regex;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Error;

use self::regex::Regex;

pub fn write_item(item: String, fname: &String) -> Result<(), Error> {
    let res = OpenOptions::new().create(true).append(true).open(fname);
    match res {
        Ok(f_) => {
            let mut f = f_;
            f.write_all(format!("<item>{}</item>", item).as_bytes())?;
            f.sync_data()?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn read_items(fname: String, attrs: Vec<&str>) -> Result<(), Error> {
    let xml_tag_regex =
        |tag: String| -> Regex { Regex::new(&format!(r"<{}>(.+?)</{}>", tag, tag)).unwrap() };
    let res = OpenOptions::new().read(true).open(fname);
    match res {
        Ok(f_) => {
            let mut f = f_;
            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("something went wrong reading the file");
            for item in xml_tag_regex("item".to_string()).captures_iter(&contents) {
                for attr in attrs.iter() {
                    let re = xml_tag_regex(attr.to_string());
                    if let Some(cap) = re.captures(&item[1]) {
                        println!("{}: {}", attr, &cap[1]);
                    }
                }
                println!("");
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}
