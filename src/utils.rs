extern crate crypto;
extern crate regex;
extern crate hex;

use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use self::regex::Regex;
use std::error;
use std::mem;
use std::str;

extern crate byteorder;

use self::byteorder::ByteOrder;
use self::byteorder::{LittleEndian, WriteBytesExt};

const HASH_BYTES: usize = 20;
const HEADER_BYTES: usize = HASH_BYTES + 2 * mem::size_of::<u64>();

pub struct ItemHeader {
    pub item_length: usize,
    pub item_pos: u64,
    pub hash: [u8; HASH_BYTES],
}

fn create_header(item: &String, fname: &String) -> Result<ItemHeader, Box<error::Error>> {
    let mut hasher = Sha1::new();
    hasher.input_str(item);
    let mut bytes = vec![0u8; HASH_BYTES];
    hasher.result(bytes.as_mut_slice());
    let hash = &mut [0u8; HASH_BYTES];
    hash.copy_from_slice(&bytes);

    Ok(ItemHeader {
        item_length: item.len(),
        item_pos: fs::metadata(fname)?.len(),
        hash: *hash,
    })
}

fn header_to_bytes(header: ItemHeader) -> [u8; HEADER_BYTES] {
    let mut b_len = [0u8; mem::size_of::<u64>()];
    b_len
        .as_mut()
        .write_u64::<LittleEndian>(header.item_length as u64)
        .expect("unable to write");

    let mut b_pos = [0u8; mem::size_of::<u64>()];
    b_pos
        .as_mut()
        .write_u64::<LittleEndian>(header.item_pos)
        .expect("unable to write");

    let res = &mut b_len.to_vec();
    res.append(&mut b_pos.to_vec());
    res.append(&mut header.hash.to_vec());

    let mut array = [0; HEADER_BYTES];
    let res = &res[..array.len()];
    array.copy_from_slice(res);
    array
}

fn bytes_to_header(header_bytes: &[u8; HEADER_BYTES]) -> Result<ItemHeader, Box<error::Error>> {
    let b_len: [u8; mem::size_of::<u64>()] =
        clone_into_array(&header_bytes[0..(mem::size_of::<u64>())]);
    let b_pos: [u8; mem::size_of::<u64>()] =
        clone_into_array(&header_bytes[mem::size_of::<u64>()..(2 * mem::size_of::<u64>())]);
    let b_hash: [u8; HASH_BYTES] = clone_into_array(
        &header_bytes[(2 * mem::size_of::<u64>())..(HASH_BYTES + 2 * mem::size_of::<u64>())],
    );
    Ok(ItemHeader {
        item_length: LittleEndian::read_u64(&b_len) as usize,
        item_pos: LittleEndian::read_u64(&b_pos),
        hash: b_hash,
    })
}

pub fn count_items(aux_fname: &String) -> Result<u32, Box<error::Error>> {
    let out: u32 = (fs::metadata(aux_fname)?.len() as u32) / (HEADER_BYTES as u32);
    Ok(out)
}

fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

pub fn read_aux_cell(
    offset: usize,
    reader: &mut BufReader<fs::File>,
) -> Result<ItemHeader, Box<error::Error>> {
    let mut buf = [0u8; HEADER_BYTES];
    reader.seek(io::SeekFrom::Start((offset * HEADER_BYTES) as u64))?;
    reader.read_exact(&mut buf)?;
    Ok(bytes_to_header(&buf)?)
}

pub fn write_item(
    item: String,
    fname: &String,
    aux_fname: &String,
) -> Result<(), Box<error::Error>> {
    let mut f = OpenOptions::new().create(true).append(true).open(fname)?;
    let mut aux_f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(aux_fname)?;

    aux_f.write_all(&header_to_bytes(create_header(&item, fname)?))?;
    aux_f.sync_data()?;

    f.write_all(item.as_bytes())?;
    f.sync_data()?;
    Ok(())
}

pub fn read_item(
    header: ItemHeader,
    reader: &mut BufReader<fs::File>,
    regexes: &Vec<Regex>,
    cdata_re: &Regex,
) -> Result<(), Box<error::Error>> {
    reader.seek(io::SeekFrom::Start(header.item_pos))?;
    let mut buf = Vec::with_capacity(header.item_length);
    unsafe {
        buf.set_len(header.item_length);
    }
    reader.read_exact(&mut buf)?;
    print_attrs(&str::from_utf8(&buf)?.to_string(), regexes, cdata_re, header.hash)?;
    Ok(())
}

fn print_attrs(
    item_txt: &String,
    regexes: &Vec<Regex>,
    cdata_re: &Regex,
    hash: [u8; HASH_BYTES],
) -> Result<(), Box<error::Error>> {
    for re in regexes.iter() {
        if let Some(cap) = re.captures(item_txt) {
            let tmp = cdata_re.replace_all(&cap[1], r"$1").to_string();
            println!("({}) {}", &hex::encode(hash)[..7], tmp);
        }
    }
    Ok(())
}
