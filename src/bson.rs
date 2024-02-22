use std::io::{Error, ErrorKind};

use bson::Document;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AccVersion1 {
    v: i32,
    //other stuff
    d: bson::Binary,
}
impl AccVersion1 {
    pub fn new(data: &[u8]) -> AccVersion1 {
        AccVersion1 {
            v: 1,
            d: bson::Binary {
                subtype: bson::spec::BinarySubtype::Generic,
                bytes: Vec::from(data),
            },
        }
    }
}/*
impl Serialize for AccVersion1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("AccVersion1", 2)?;
        state.serialize_field("v", &self.v)?;
        state.serialize_field("d", &self.d)?;
        state.end()
    }
}*/
#[derive(PartialEq, Debug)]
pub enum PacketVersion {
    V1(AccVersion1),
}

pub(crate) fn encapsulate(acc_enum: &PacketVersion) -> Result<Vec<u8>, std::io::Error> {
    let acc_struct = match acc_enum {
        PacketVersion::V1(structure) => structure,
    };
    Ok(bson::to_vec(&acc_struct).unwrap())
}

fn reveal(bson_vec: Vec<u8>) -> Result<PacketVersion, std::io::Error> {
    let doc: Document = bson::from_slice(&bson_vec).unwrap();
    let version = doc.get_i32("v");
    let result = match version {
        Ok(0) | Ok(1) => {
            let acc_struct: AccVersion1 = bson::from_document(doc).unwrap();
            PacketVersion::V1(acc_struct)
        }
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
        let pkt = PacketVersion::V1(AccVersion1::new(
            b"somebody once told me the world was gonna roll me".to_vec(),
        ));
        let result = reveal(encapsulate(&pkt).unwrap()).unwrap();
        assert_eq!(pkt, result)
    }
}
