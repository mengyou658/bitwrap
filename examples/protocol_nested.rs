use {
    bitwrap_extra::{
        BitWrap,
        BitWrapExt,
    },
};

#[derive(Default, Debug, BitWrap)]
struct ControlPacketBody {
    // single bit field
    #[bitfield(8)]
    data1: u8,
    // single bit field
    #[bitfield(8)]
    data2: u8,
    // single bit field
    #[bitfield(8)]
    data3: u8,
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
    #[bitfield(16, name = data_len, value = self.data.len())]
    data_len: u16,
    // get slice of `data_len` bytes and call BitWrapExt method for Vec<T>
    // where T is u8 or with implemented BitWrapExt + Default traits
    #[bitfield(data_len)]
    data: ControlPacketBody,

    // bit field as boolean. 0 is false, otherwise is true
    #[bitfield(8, name = crc, value = self.crc())]
    crc: u8,
}

impl ControlPacket {
    pub fn crc(&self) -> u16 {
        // crc ...
        return 3;
    }
}

fn main() {
    const DATA: &[u8] = &[1, 2, 0, 3, 1, 2, 3, 2];

    let mut packet = ControlPacket::default();
    packet.id = 1;
    packet.dataType1 = 2;
    packet.dataType2 = 2;
    packet.data = ControlPacketBody {
        data1: 1,
        data2: 2,
        data3: 3
    };
    packet.crc = 2;

    let len: usize = DATA.len();
    let mut buffer = vec![0 as u8; len];

    let result = packet.pack().unwrap();
    println!("res {:?}", result);
    println!("res {:?}", buffer);


    let mut packet = ControlPacket::default();
    let result = packet.unpack(DATA).unwrap();
    println!("res {:?}", result);
    println!("res {:?}", packet);

}
