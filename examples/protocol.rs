use {
    core::convert::{
        TryFrom,
        Infallible,
    },
    std::net::Ipv4Addr,
    bitwrap::{
        BitWrap,
        BitWrapExt,
        BitWrapError,
    },
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
    #[bitfield(8)]
    dataType: u8,

    // virtual field with optn `name` to define buffer length
    #[bitfield(16, name = data_len, value = self.data.len())]
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
    packet.dataType = 2;
    packet.data = vec![1,2,3];
    packet.crc = 2;

    let len: usize = 8;
    let mut buffer = vec![0 as u8; len];

    let result = packet.pack(&mut buffer).unwrap();
    println!("res {:?}", result);
    println!("res {:?}", buffer);

    const DATA: &[u8] = &[1, 2, 0, 3, 1, 2, 3, 2];

    let mut packet = ControlPacket::default();
    let result = packet.unpack(DATA).unwrap();
    println!("res {:?}", result);
    println!("res {:?}", packet);
    println!("res {:?}", packet.data_len);

}
