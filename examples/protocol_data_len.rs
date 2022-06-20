use {
    bitwrap_extra::{
        BitWrap,
        BitWrapExt,
    },
};

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
    #[bitfield(24, name = data_len, value = self.data.len())]
    data_len: u32,
    // get slice of `data_len` bytes and call BitWrapExt method for Vec<T>
    // where T is u8 or with implemented BitWrapExt + Default traits
    #[bitfield(data_len)]
    data: Vec<u8>,

    // bit field as boolean. 0 is false, otherwise is true
    #[bitfield(8)]
    crc: u8,
}

//
// impl bitwrap_extra::BitWrapExt for ControlPacket {
//     fn len(&self) -> usize {
//         let mut length: usize = 0;
//         length += (8usize) as usize;
//         length += (4usize) as usize;
//         length += (4usize) as usize;
//         length += (24usize) as usize;
//         let data_len = (self.data.len()) as usize;
//         length += data_len * 8 as usize;
//         length += (8usize) as usize;
//         (length / 8) as usize
//     }
//     fn pack(&self) -> Result<Vec<u8>, bitwrap_extra::BitWrapError> {
//         use core::convert::TryFrom as _;
//         let len = self.len() as usize;
//         let mut dst = vec![0 as u8; len];
//         let mut offset: usize = 0;
//         if 1usize + offset > dst.len() {
//             return Err(bitwrap_extra::BitWrapError);
//         }
//         dst[offset] = 0;
//         let value: u8 = u8::try_from(self.id)?;
//         dst[offset] |= (value as u8) & 255u8;
//         offset += 1;
//         if 1usize + offset > dst.len() {
//             return Err(bitwrap_extra::BitWrapError);
//         }
//         dst[offset] = 0;
//         let value: u8 = u8::try_from(self.dataType1)?;
//         dst[offset] |= ((value as u8) & 15u8) << 4usize;
//         let value: u8 = u8::try_from(self.dataType2)?;
//         dst[offset] |= (value as u8) & 15u8;
//         offset += 1;
//         if 3usize + offset > dst.len() {
//             return Err(bitwrap_extra::BitWrapError);
//         }
//         dst[offset] = 0;
//         let value = (self.data.len()) as u32;
//         let data_len = value;
//         dst[offset] |= ((value >> 16usize) as u8) & 255u8;
//         offset += 1;
//         dst[offset] = 0;
//         dst[offset] |= ((value >> 8usize) as u8) & 255u8;
//         offset += 1;
//         dst[offset] = 0;
//         dst[offset] |= (value as u8) & 255u8;
//         offset += 1;
//         let value: u32 = u32::try_from(self.data_len)?;
//         let limit = offset + (data_len) as usize;
//         if dst.len() >= limit {
//             let tmp = self.data.pack(&mut dst[offset..limit])?;
//             let tmp_len = tmp.len();
//             dst[offset..offset + tmp_len].clone_from_slice(tmp.as_slice());
//             offset += tmp_len;
//         } else {
//             return Err(bitwrap_extra::BitWrapError);
//         }
//         if 1usize + offset > dst.len() {
//             return Err(bitwrap_extra::BitWrapError);
//         }
//         dst[offset] = 0;
//         let value: u8 = u8::try_from(self.crc)?;
//         dst[offset] |= (value as u8) & 255u8;
//         offset += 1;
//         Ok(dst)
//     }
//     fn unpack(&mut self, src: &[u8]) -> Result<usize, bitwrap_extra::BitWrapError> {
//         use core::convert::TryFrom as _;
//         let mut offset: usize = 0;
//         if 1usize + offset > src.len() {
//             return Err(bitwrap_extra::BitWrapError);
//         }
//         let mut value: u8 = 0;
//         value |= (src[offset] & 255u8) as u8;
//         offset += 1;
//         self.id = u8::try_from(value)?;
//         if 1usize + offset > src.len() {
//             return Err(bitwrap_extra::BitWrapError);
//         }
//         let mut value: u8 = 0;
//         value |= ((src[offset] >> 4usize) & 15u8) as u8;
//         self.dataType1 = u8::try_from(value)?;
//         let mut value: u8 = 0;
//         value |= (src[offset] & 15u8) as u8;
//         offset += 1;
//         self.dataType2 = u8::try_from(value)?;
//         if 3usize + offset > src.len() {
//             return Err(bitwrap_extra::BitWrapError);
//         }
//         let mut value: u32 = 0;
//         value |= ((src[offset] & 255u8) as u32) << 16usize;
//         offset += 1;
//         value |= ((src[offset] & 255u8) as u32) << 8usize;
//         offset += 1;
//         value |= (src[offset] & 255u8) as u32;
//         offset += 1;
//         let data_len = value;
//         self.data_len = u32::try_from(value)?;
//         let limit = offset + (data_len) as usize;
//         if src.len() >= limit {
//             offset += self.data.unpack(&src[offset..limit])?;
//         } else {
//             return Err(bitwrap_extra::BitWrapError);
//         }
//         if 1usize + offset > src.len() {
//             return Err(bitwrap_extra::BitWrapError);
//         }
//         let mut value: u8 = 0;
//         value |= (src[offset] & 255u8) as u8;
//         offset += 1;
//         self.crc = u8::try_from(value)?;
//         Ok(offset)
//     }
// }

fn main() {
    const DATA: &[u8] = &[1, 2, 0, 0, 3, 1, 2, 3, 2];

    let mut packet = ControlPacket::default();
    packet.id = 1;
    packet.dataType1 = 2;
    packet.dataType2 = 2;
    packet.data = vec![1,2,3];
    packet.crc = 2;

    let result = packet.pack().unwrap();
    println!("res {:?}", result);


    let mut packet = ControlPacket::default();
    let result = packet.unpack(DATA).unwrap();
    println!("res {:?}", result);
    println!("res {:?}", packet);

}
