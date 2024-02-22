use rand::Rng;
use std::io::{Error, ErrorKind};

const MIN_BYTES_PADDING: usize = 2;
const MAX_BYTES_PADDING: usize = 32;

/// Encapsulates the given BSON packet with a random number (between 2 and 32 inclusive) of random bytes before and after.
/// # Arguments
/// * `pkt` - A reference to a BSON packet.
/// # Returns
/// The encapsulated packet in Vec<u8> form.
/// # Errors
/// Returns an error if there is an error during the generation of random bytes or packet encapsulation
pub(crate) fn encapsulate(pkt: &[u8]) -> Vec<u8> {
    let mut rng = rand::thread_rng(); // use the random number generator to generate our random numbers and bytes
    let r1 = rng.gen_range(MIN_BYTES_PADDING..=MAX_BYTES_PADDING); // length of random bytes before the BSON data
    let r2 = rng.gen_range(MIN_BYTES_PADDING..=MAX_BYTES_PADDING); // length of random bytes after the BSON data

    // vectors containing the random bytes for the front and back padding of the BSON data
    let mut front_pad = Vec::with_capacity(r1 + 1);
    let mut back_pad = Vec::with_capacity(r2 + 1);

    while front_pad.len() < r1 {
        let rand_byte: u8 = rng.gen();

        if rand_byte != '{' as u8 {
            front_pad.push(rand_byte);
        }
    }

    while back_pad.len() < r2 {
        let rand_byte: u8 = rng.gen();

        if rand_byte != '}' as u8 {
            back_pad.push(rand_byte);
        }
    }

    front_pad.push('{' as u8);
    back_pad.insert(0, '}' as u8);

    // define a new vector to add the front pad, the packet, and finally the back pad
    let mut encapsulated_pkt = Vec::with_capacity(pkt.len() + front_pad.len() + back_pad.len());
    encapsulated_pkt.extend(front_pad);
    encapsulated_pkt.extend(pkt);
    encapsulated_pkt.extend(back_pad);

    encapsulated_pkt
}

/// Strips the padding from an unencrypted packet.
/// # Arguments
/// * `pkt` - A reference to a BSON packet with randomized padding.
/// # Returns
/// The revealed BSON in Vec<u8> form if successful.
/// # Errors
/// Returns an error if the padding is not valid (does not have the correct braces) or
/// if there is an error during the extraction of BSON data.
fn reveal(pkt: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    // initialize bson_front_index & bson_back_index with a default of -1 to indicate they hasn't been changed
    let mut bson_front_index: i32 = -1;
    let mut bson_back_index = -1;

    // loop through the packet and find the index of the beginning of the BSON data.
    for (index, &element) in pkt.iter().enumerate().take(MAX_BYTES_PADDING + 1) {
        if element == b'{' {
            // if the opening brace is not within the minimum and maximum bytes of padding, the padding is not valid
            if index < MIN_BYTES_PADDING || index > MAX_BYTES_PADDING {
                return Err(Error::from(ErrorKind::InvalidData));
            } else {
                bson_front_index = (index + 1) as i32; // store the index of the bracket for later use
                break;
            }
        }
    }

    // if the bson_front_index hasn't been changed, we were unable to find an open bracket and therefore need to throw an error
    if bson_front_index == -1 {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    // loop through the packet backwards to find the end of the BSON data
    for (rev_index, &element) in pkt.iter().rev().enumerate().take(MAX_BYTES_PADDING + 1) {
        if element == b'}' {
            // if the closing brace is not within the minimum and maximum bytes of padding, the padding is not valid
            if rev_index < MIN_BYTES_PADDING || rev_index > MAX_BYTES_PADDING {
                return Err(Error::from(ErrorKind::InvalidData));
            } else {
                let index = pkt.len() - 1 - rev_index; // since the index is now reversed, we have to turn it back into the normal index for future use
                bson_back_index = index as i32;
                break;
            }
        }
    }

    // if the bson_back_index hasn't been changed, we were unable to find a closed bracket and therefore need to throw an error
    if bson_back_index == -1 {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    // the revealed packet is just the bytes between the opening bracket and closing bracket
    let revealed_pkt = pkt[bson_front_index as usize..bson_back_index as usize].to_vec();

    Ok(revealed_pkt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bson_wrap_unwrap() {
        let pkt = b"This is sample BSON data.";
        assert_eq!(reveal(&encapsulate(pkt)).unwrap(), pkt.to_vec());
    }
}
