// No definitions yet.
// Need to define the encapsulate and reveal functions.
// Because BSON depends on the protocol version,
// need to develop an Enum<Struct, Struct, ...> to
// work based on the protocol version.
use bson::{bson, Binary, Bson, Document};
use bson::spec::BinarySubtype;
use serde::{Deserialize};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::io::{Error, ErrorKind};

#[derive(Deserialize, PartialEq, Debug)]
struct AccVersion1 {
    v: i32,
    //other stuff
    d: bson::Binary,
}
impl AccVersion1 {
    fn new(data: Vec<u8>) -> AccVersion1 {
        AccVersion1 { 
            v: 1, 
            d: bson::Binary{subtype: bson::spec::BinarySubtype::Generic,
            bytes: data}
        } 
    }
}
impl Serialize for AccVersion1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
                let mut state = serializer.serialize_struct("AccVersion1", 2)?;
                state.serialize_field("v", &self.v)?;
                state.serialize_field("d", &self.d)?;
                state.end()
    }
}
#[derive(PartialEq, Debug)]
enum PacketVersion {
    V1(AccVersion1),
}

fn encapsulate(acc_enum: &PacketVersion) -> Result<Vec<u8>, std::io::Error> {
    let acc_struct = match acc_enum {
        PacketVersion::V1(structure) => structure,
        _ => return Err(Error::from(ErrorKind::InvalidData)),
    };
    Ok(bson::to_vec(&acc_struct).unwrap())
}

fn reveal(bson_vec: Vec<u8>) -> Result<PacketVersion, std::io::Error> {
    let doc: Document = bson::from_slice(&bson_vec[..]).unwrap();
    let version = doc.get_i32("v");
    let result = match version {
        Ok(1) => {
            let acc_struct: AccVersion1 = bson::from_document(doc).unwrap();
            PacketVersion::V1(acc_struct)
        },
        Ok(0) => {
            let acc_struct: AccVersion1 = bson::from_document(doc).unwrap();
            PacketVersion::V1(acc_struct)
        },
        Ok(_) => return Err(Error::from(ErrorKind::Unsupported)),
        Err(error) => panic!("Error: {}", error),
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bson::{doc, spec::BinarySubtype, Document};

    #[test]
    fn bson_test() {
        let pkt = PacketVersion::V1(AccVersion1::new(b"somebody once told me the world was gonna roll me".to_vec()));
        let result = reveal(encapsulate(&pkt).unwrap()).unwrap();
        assert_eq!(pkt, result)
    }
}
