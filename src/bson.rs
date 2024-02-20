// No definitions yet.
// Need to define the encapsulate and reveal functions.
// Because BSON depends on the protocol version,
// need to develop an Enum<Struct, Struct, ...> to
// work based on the protocol version.
use bson::{bson, Binary, Bson};
use bson::spec::BinarySubtype;
use serde::{Deserialize};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::io::{Error, ErrorKind};

#[derive(Deserialize)]
struct AccVersion1 {
    v: i32,
    //other stuff
    d: bson::Binary,
}
impl AccVersion1 {
    fn new(d: bson::Binary) -> AccVersion1 {
        AccVersion1 { v: 1, d: d.clone() }
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
enum PacketVersion {
    V1(AccVersion1),
}

fn encapsulate(acc_struct: PacketVersion) -> Result<Vec<u8>, std::io::Error> {
    unimplemented!()
}

fn reveal(bson_doc: Vec<u8>) -> Result<PacketVersion, std::io::Error> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bson::{doc, spec::BinarySubtype};

    #[test]
    fn bson_test() {
        let pkt = AccVersion1::new(bson::Binary {
            subtype: bson::spec::BinarySubtype::Generic,
            bytes: b"somebody once told me".to_vec(),
        });
        let bson = bson::to_vec(&pkt);
        print!("{:x?}", bson);
    }
}
