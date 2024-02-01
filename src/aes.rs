/// Generates AES-encrypted packet from key.
/// Returns the encrypted packet with appended nonce,
/// which is really only the first 96 bits of the last 16 bytes.
fn encapsulate(pkt: &[u8], key: &[u8]) -> Vec<u8> {
    unimplemented!()
}

/// Attempts to decrypt AES with a given key.
/// If this fails, returns Err.
/// On success, returns Ok(Vec<u8>) containing the packet.
fn reveal(pkt: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    unimplemented!()
}
