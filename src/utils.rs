extern crate crypto;
extern crate regex;

use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::SeekFrom;
use std::str::Utf8Error;

use self::regex::Regex;
use std::error;
use std::str;
use std::str::FromStr;

struct ItemHeader {
    length: usize,
    hash: String,
}

fn create_header(item: &String) -> ItemHeader {
    let mut hasher = Sha1::new();
    hasher.input_str(item);
    ItemHeader {
        length: item.len(),
        hash: hasher.result_str(),
    }
}

fn header_to_str(header: ItemHeader) -> String {
    format!("[{},{}]", header.length, header.hash)
}

fn str_to_header(header_str: &Vec<u8>) -> Result<ItemHeader, Utf8Error> {
    let cnt = str::from_utf8(&header_str[1..(header_str.len() - 1)])?;
    let vals = cnt.split(",").collect::<Vec<&str>>();
    Ok(ItemHeader {
        length: usize::from_str(vals[0]).unwrap(),
        hash: vals[1].to_string(),
    })
}

pub fn write_item(item: String, fname: &String) -> Result<(), Error> {
    let mut f = OpenOptions::new().create(true).append(true).open(fname)?;
    f.write_all(format!("{}{}", header_to_str(create_header(&item)), item).as_bytes())?;
    f.sync_data()?;
    Ok(())
}

pub fn read_items(fname: String, attrs: Vec<&str>) -> Result<(), Error> {
    let f = OpenOptions::new().read(true).open(&fname)?;
    let xml_tag_regex =
        |tag: String| -> Regex { Regex::new(&format!(r"<{}>(.+?)</{}>", tag, tag)).unwrap() };
    let regexes: Vec<Regex> = attrs
        .iter()
        .into_iter()
        .map(|x| xml_tag_regex(x.to_string()))
        .collect();
    let mut reader = BufReader::new(f);
    let metadata = fs::metadata(fname)?;
    print_all_items(&mut reader, 0, metadata.len(), &regexes);
    Ok(())
}

fn print_attrs(item_txt: &String, regexes: &Vec<Regex>, header: &ItemHeader) {
    for re in regexes.iter() {
        if let Some(cap) = re.captures(item_txt) {
            println!("({}): {}", &header.hash[..7], &cap[1]);
        }
    }
}

fn print_all_items(
    reader: &mut BufReader<File>,
    offset: usize,
    file_len: u64,
    regexes: &Vec<Regex>,
) -> Result<(), Box<error::Error>> {
    let mut buf = vec![];
    let hlen = reader.read_until(b']', &mut buf)?;
    let header = str_to_header(&buf)?;
    reader.seek(SeekFrom::Start((hlen + offset) as u64))?;
    let mut buf2 = Vec::with_capacity(header.length);
    unsafe {
        buf2.set_len(header.length);
    }
    reader.read_exact(&mut buf2);
    print_attrs(
        &str::from_utf8(&buf2).unwrap().to_string(),
        regexes,
        &header,
    );
    reader.seek(SeekFrom::Start((offset + hlen + header.length) as u64));
    if offset + (hlen as usize) + header.length < (file_len as usize) {
        print_all_items(
            reader,
            offset + (hlen as usize) + header.length,
            file_len,
            regexes,
        )
    } else {
        Ok(())
    }
}
