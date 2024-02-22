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

// According to RFC8452, this should never yield a "false packet"
// (unlike OpenSSL's CBC mode), as GCM-SIV is AEAD
pub fn decrypt_packet(pkt: &[u8], key: &[u8]) -> Result<PacketVersion, Box<dyn std::error::Error>> {
    let aesd = aes::reveal(pkt, key)?;
    let padd = padding::reveal(&aesd)?;
    Ok(bson::reveal(&padd)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aes_gcm_siv::KeyInit;

    #[test]
    fn basic_full_stack_packet() {
        let key = aes_gcm_siv::Aes256GcmSiv::generate_key(aes_gcm_siv::aead::OsRng);
        let pkt = b"ACC: the Aggressive Circumvention of Censorship project".to_vec();
        // We allow irrefutable let patterns for now because we only have V1
        // This won't be necessary later on - NOTE: remove when V2 releases
        #[allow(irrefutable_let_patterns)]
        if let PacketVersion::V1(d) = decrypt_packet(&encrypt_packet(&pkt, &key, 1).unwrap(), &key).unwrap() {
            assert_eq!(pkt, d.d.bytes);
        } else {
            assert!(false);
        }
    }
    #[test]
    fn basic_negative() {
        let key = aes_gcm_siv::Aes256GcmSiv::generate_key(aes_gcm_siv::aead::OsRng);
        let key_bad = aes_gcm_siv::Aes256GcmSiv::generate_key(aes_gcm_siv::aead::OsRng);
        let pkt = b"ACC: the Aggressive Circumvention of Censorship project".to_vec();
        if let Ok(_) = decrypt_packet(&encrypt_packet(&pkt, &key, 1).unwrap(), &key_bad) {
            assert!(false);
        }
    }
}
