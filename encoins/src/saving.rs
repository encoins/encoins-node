use csv;
use std::error::Error;
use std::fs::OpenOptions;
use serde::Serialize;
use crate::base_types::Transaction;

pub fn write_to_file(path : &str, transaction : &Transaction ) -> Result<(), Box<dyn Error>>
{
    let mut file = OpenOptions::new().write(true).create(true).append(true).open(path).unwrap();
    let mut writer = csv::Writer::from_writer(file);
    writer.serialize(transaction)?;
    writer.flush()?;

    Ok(())
}