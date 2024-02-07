// No definitions yet.
// Need to define the encapsulate and reveal functions.
// Because BSON depends on the protocol version,
// need to develop an Enum<Struct, Struct, ...> to
// work based on the protocol version.
use bson::{bson, Bson};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize)]
struct AccVersion1 {
    v: u32,
    //other stuff
    d: Vec<u8>,
}
impl AccVersion1 {
    fn new(d: &Vec<u8>) -> AccVersion1 {
        AccVersion1 { v: 1, d: d.clone() }
    }
}

enum PacketVersion {
    V1(AccVersion1),
}

fn encapsulate(acc_struct: PacketVersion) -> Result<Vec<u8>, std::io::Error> {
    let pkt = match acc_struct {
        PacketVersion::V1(pkt) => pkt,
        _ => return Err(Error::from(ErrorKind::Unsupported)),
    };
    let bson_vec = bson::to_vec(&pkt).unwrap();
    Ok(bson_vec)
}

fn reveal(bson_doc: Vec<u8>) -> Result<PacketVersion, std::io::Error> {
    //let bson_data: Bson = bson::from_slice(&bson_doc[..]).unwrap();
    //if()
    unimplemented!()
}
