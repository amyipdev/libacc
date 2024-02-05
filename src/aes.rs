use std::io::{Error, ErrorKind};

use aes_gcm_siv::{
    aead::{Aead, OsRng},
    Aes256GcmSiv, KeyInit, Nonce,
};
use rand::RngCore;

/// Generates AES-encrypted packet from key.
/// Returns the encrypted packet with appended nonce,
/// which is really only the first 96 bits of the last 16 bytes.
fn encapsulate(pkt: &[u8], key: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    if key.len() != 32 {
        return Err(Error::from(ErrorKind::InvalidInput));
    }
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let mut nonce_raw: [u8; 12] = [0u8; 12];
    rng.fill_bytes(&mut nonce_raw);
    let cipher = Aes256GcmSiv::new(key.into());
    let nonce = Nonce::from_slice(&nonce_raw);
    let mut cpt: Vec<u8> = match cipher.encrypt(nonce, pkt) {
        Ok(v) => v,
        Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
    };
    cpt.extend(&nonce_raw);
    Ok(cpt)
}

/// Attempts to decrypt AES with a given key.
/// If this fails, returns Err.
/// On success, returns Ok(Vec<u8>) containing the packet.
fn reveal(pkt: &[u8], key: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    if key.len() != 32 {
        return Err(Error::from(ErrorKind::InvalidInput));
    }
    let l0: usize = pkt.len();
    let nonce = Nonce::from_slice(&pkt[l0 - 12..]);
    let pkt = &pkt[..l0 - 12];
    let cipher = Aes256GcmSiv::new(key.into());
    match cipher.decrypt(nonce, pkt) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::from(ErrorKind::InvalidData)),
    }
    //unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enc_rev_pkts_aes256_single_rand() {
        let key = Aes256GcmSiv::generate_key(OsRng);
        let pkt = b"This is a sample packet for encryption with AES256-GCM-SIV!";
        assert_eq!(reveal(&encapsulate(pkt, &key).unwrap(), &key).unwrap(), pkt);
    }
}
