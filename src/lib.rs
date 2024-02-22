mod aes;
mod bson;
mod padding;

use std::io::{Error, ErrorKind};

pub use bson::{PacketVersion, AccVersion1};

pub fn encrypt_packet(pkt: &[u8], key: &[u8], vsn: i32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let pv = match vsn {
        0 | 1 => PacketVersion::V1(AccVersion1::new(pkt)),
        _ => return Err(Box::new(Error::from(ErrorKind::InvalidInput)))
    };
    Ok(aes::encapsulate(&padding::encapsulate(&bson::encapsulate(&pv)?), key)?)
}
