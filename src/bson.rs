// No definitions yet.
// Need to define the encapsulate and reveal functions.
// Because BSON depends on the protocol version,
// need to develop an Enum<Struct, Struct, ...> to
// work based on the protocol version.
use bson::{bson, Bson};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ACCV1Struct {
    v: u32,
    //other stuff
    d: Vec<u8>,
}
impl ACCV1Struct {
    fn new(d: &Vec<u8>) -> ACCV1Struct {
        ACCV1Struct { v: 1, d: d.clone() }
    }
}

fn encapsulate(pkt: Vec<u8>) -> Vec<u8> {
    let acc_struct: ACCV1Struct = ACCV1Struct::new(&pkt);
    let bson_vec = bson::to_vec(&acc_struct).unwrap();
    bson_vec
}

fn reveal(bson_doc: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encapsulate_test() {
        let pkt: Vec<u8> = Vec::from(b"Hello Darkness my old friend");
        println!("pkt is {:x?}", pkt);
        let acc_struct: ACCV1Struct = ACCV1Struct::new(&pkt);
        println!("acc ver is {}", acc_struct.v);
        println!("acc data is {:x?}", acc_struct.d);
        let encapsulated = encapsulate(pkt);
        println!("encapsulated is {:x?}", encapsulated);
    }
}
