use {
    core::convert::{
        TryFrom,
        Infallible,
    },
    bitwrap_extra::{
        BitWrap,
        BitWrapExt,
        BitWrapError,
    },
    std::convert::TryInto,
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Variant { Value55, ValueAA }

impl Default for Variant {
    fn default() -> Self { Variant::Value55 }
}

impl TryFrom<u8> for Variant {
    type Error = BitWrapError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x55 => Ok(Variant::Value55),
            0xAA => Ok(Variant::ValueAA),
            _ => Err(BitWrapError),
        }
    }
}

impl TryFrom<Variant> for u8 {
    type Error = Infallible;
    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        match value {
            Variant::Value55 => Ok(0x55),
            Variant::ValueAA => Ok(0xAA),
        }
    }
}

#[derive(Default, Debug, BitWrap)]
struct ControlPacket {
    // single bit field
    #[bitfield(8)]
    id: u8,

    // bit field as boolean. 0 is false, otherwise is true
    #[bitfield(4)]
    dataType1: u8,
    #[bitfield(4)]
    dataType2: u8,

    // virtual field with optn `name` to define buffer length
    #[bitfield(16, name = data_len, value = self.data.len(), pack = LE, unpack = LE)]
    data_len: u16,
    // get slice of `data_len` bytes and call BitWrapExt method for Vec<T>
    // where T is u8 or with implemented BitWrapExt + Default traits
    #[bitfield(data_len)]
    data: Vec<u8>,

    // bit field as boolean. 0 is false, otherwise is true
    #[bitfield(8)]
    crc: u8,
}

fn main() {

    let mut packet = ControlPacket::default();
    packet.id = 1;
    packet.dataType1 = 2;
    packet.dataType2 = 2;
    packet.data = vec![1,2,3];
    packet.crc = 2;

    let len: usize = 8;
    let mut buffer = vec![0 as u8; len];

    let result = packet.pack(&mut buffer).unwrap();
    println!("res {:?}", result);
    println!("res {:?}", buffer);

    const DATA: &[u8] = &[1, 2, 3, 0, 1, 2, 3, 2];

    let mut packet = ControlPacket::default();
    let result = packet.unpack(DATA).unwrap();
    println!("res {:?}", result);
    println!("res {:?}", packet);
    let test = 3 as u16;
    println!("res {:?}", test.to_be_bytes());
    let val = test.to_le_bytes();
    let (int_bytes, rest) = val.split_at(std::mem::size_of::<u16>());
    println!("res {:?}", rest);
    println!("res {:?}", u16::from_le_bytes(val[..].try_into().unwrap()));
    for i in val {
        println!("val: {}", i)
    }
    println!("res {:?}", test.to_le_bytes());
    println!("res {:?}", packet.data_len);

}
