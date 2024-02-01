// Applies new padding to a BSON packet.
// Returns the padded packet.
fn encapsulate(pkt: &[u8]) -> Vec<u8> {
    unimplemented!()
}

/// Strips the padding from an unencrypted packet.
/// Returns Ok(Vec<u8>) containing the BSON if successful.
/// Returns Err if the packet is invalid.
fn reveal(pkt: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    unimplemented!()
}
