extern crate regex;

use self::regex::Regex;
use super::utils;
use std::error;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::{Read, Write};

pub fn alias(feed_alias: String, feed_url: String, fname: String) -> Result<(), Box<error::Error>> {
    if !utils::is_url(&feed_url) {
        return Err("Invalid url".into());
    }
    match OpenOptions::new().read(true).open(&fname) {
        Ok(f) => {
            let mut data = String::new();
            let mut br = BufReader::new(f);
            br.read_to_string(&mut data).expect("Unable to read string");

            let re = Regex::new(&format!("{}=.+?\n", &feed_alias)).unwrap();
            data = re.replace_all(&data, "").to_string();

            data.push_str(&format!("{}={}\n", feed_alias, feed_url));

            let mut f = OpenOptions::new().write(true).truncate(true).open(&fname)?;
            f.write_all(&data.as_bytes())?;
            f.sync_data()?;
        }
        Err(_e) => {
            let mut f = OpenOptions::new().write(true).create(true).open(&fname)?;
            let mut data = String::new();
            data.push_str(&format!("{}={}\n", feed_alias, feed_url));
            f.write_all(data.as_bytes())?;
            f.sync_data()?;
        }
    }
    Ok(())
}
