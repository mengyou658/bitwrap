#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use {
    bitwrap_extra::{BitWrap, BitWrapExt},
};
struct ControlPacket {
    #[bitfield(8)]
    id: u8,
    #[bitfield(4)]
    dataType1: u8,
    #[bitfield(4)]
    dataType2: u8,
    # [bitfield (16 , name = data_len , value = self . data . len ())]
    data_len: u16,
    #[bitfield(data_len)]
    data: Vec<u8>,
    #[bitfield(8)]
    crc: u8,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for ControlPacket {
    #[inline]
    fn default() -> ControlPacket {
        ControlPacket {
            id: ::core::default::Default::default(),
            dataType1: ::core::default::Default::default(),
            dataType2: ::core::default::Default::default(),
            data_len: ::core::default::Default::default(),
            data: ::core::default::Default::default(),
            crc: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for ControlPacket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            ControlPacket {
                id: ref __self_0_0,
                dataType1: ref __self_0_1,
                dataType2: ref __self_0_2,
                data_len: ref __self_0_3,
                data: ref __self_0_4,
                crc: ref __self_0_5,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "ControlPacket");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_0));
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "dataType1",
                    &&(*__self_0_1),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "dataType2",
                    &&(*__self_0_2),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "data_len",
                    &&(*__self_0_3),
                );
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "data", &&(*__self_0_4));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "crc", &&(*__self_0_5));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
impl bitwrap_extra::BitWrapExt for ControlPacket {
    fn pack(&self, dst: &mut [u8]) -> Result<usize, bitwrap_extra::BitWrapError> {
        use core::convert::TryFrom as _;
        let mut offset: usize = 0;
        if 1usize + offset > dst.len() {
            return Err(bitwrap_extra::BitWrapError);
        }
        dst[offset] = 0;
        let value: u8 = u8::try_from(self.id)?;
        dst[offset] |= (value as u8) & 255u8;
        offset += 1;
        if 1usize + offset > dst.len() {
            return Err(bitwrap_extra::BitWrapError);
        }
        dst[offset] = 0;
        let value: u8 = u8::try_from(self.dataType1)?;
        dst[offset] |= ((value as u8) & 15u8) << 4usize;
        let value: u8 = u8::try_from(self.dataType2)?;
        dst[offset] |= (value as u8) & 15u8;
        offset += 1;
        if 2usize + offset > dst.len() {
            return Err(bitwrap_extra::BitWrapError);
        }
        dst[offset] = 0;
        let value = (self.data.len()) as u16;
        let data_len = value;
        dst[offset] |= ((value >> 8usize) as u8) & 255u8;
        offset += 1;
        dst[offset] = 0;
        dst[offset] |= (value as u8) & 255u8;
        offset += 1;
        let value: u16 = u16::try_from(self.data_len)?;
        let limit = offset + (data_len) as usize;
        if dst.len() >= limit {
            offset += self.data.pack(&mut dst[offset..limit])?;
        } else {
            return Err(bitwrap_extra::BitWrapError);
        }
        if 1usize + offset > dst.len() {
            return Err(bitwrap_extra::BitWrapError);
        }
        dst[offset] = 0;
        let value: u8 = u8::try_from(self.crc)?;
        dst[offset] |= (value as u8) & 255u8;
        offset += 1;
        Ok(offset)
    }
    fn unpack(&mut self, src: &[u8]) -> Result<usize, bitwrap_extra::BitWrapError> {
        use core::convert::TryFrom as _;
        let mut offset: usize = 0;
        if 1usize + offset > src.len() {
            return Err(bitwrap_extra::BitWrapError);
        }
        let mut value: u8 = 0;
        value |= (src[offset] & 255u8) as u8;
        offset += 1;
        self.id = u8::try_from(value)?;
        if 1usize + offset > src.len() {
            return Err(bitwrap_extra::BitWrapError);
        }
        let mut value: u8 = 0;
        value |= ((src[offset] >> 4usize) & 15u8) as u8;
        self.dataType1 = u8::try_from(value)?;
        let mut value: u8 = 0;
        value |= (src[offset] & 15u8) as u8;
        offset += 1;
        self.dataType2 = u8::try_from(value)?;
        if 2usize + offset > src.len() {
            return Err(bitwrap_extra::BitWrapError);
        }
        let mut value: u16 = 0;
        value |= ((src[offset] & 255u8) as u16) << 8usize;
        offset += 1;
        value |= (src[offset] & 255u8) as u16;
        offset += 1;
        let data_len = value;
        self.data_len = u16::try_from(value)?;
        let limit = offset + (data_len) as usize;
        if src.len() >= limit {
            offset += self.data.unpack(&src[offset..limit])?;
        } else {
            return Err(bitwrap_extra::BitWrapError);
        }
        if 1usize + offset > src.len() {
            return Err(bitwrap_extra::BitWrapError);
        }
        let mut value: u8 = 0;
        value |= (src[offset] & 255u8) as u8;
        offset += 1;
        self.crc = u8::try_from(value)?;
        Ok(offset)
    }
}
fn main() {
    const DATA: &[u8] = &[1, 2, 0, 3, 1, 2, 3, 2];
    let mut packet = ControlPacket::default();
    packet.id = 1;
    packet.dataType1 = 2;
    packet.dataType2 = 2;
    packet.data = <[_]>::into_vec(box [1, 2, 3]);
    packet.crc = 2;
    let len: usize = DATA.len();
    let mut buffer = ::alloc::vec::from_elem(0 as u8, len);
    let result = packet.pack(&mut buffer).unwrap();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["res ", "\n"],
            &match (&result,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["res ", "\n"],
            &match (&buffer,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    let mut packet = ControlPacket::default();
    let result = packet.unpack(DATA).unwrap();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["res ", "\n"],
            &match (&result,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["res ", "\n"],
            &match (&packet,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
}
