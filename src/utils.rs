use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Error;

pub fn write_item(item: String, fname: &String) -> Result<(), Error> {
    let res = OpenOptions::new().create(true).append(true).open(fname);
    match res {
        Ok(f_) => {
            let mut f = f_;
            f.write_all(item.as_bytes())?;
            f.sync_data()?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}
