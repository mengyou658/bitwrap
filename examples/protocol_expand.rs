#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use {
    core::convert::{TryFrom, Infallible},
    bitwrap_extra::{BitWrap, BitWrapExt, BitWrapError},
    std::convert::TryInto,
};
enum Variant {
    Value55,
    ValueAA,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Variant {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Variant::Value55,) => ::core::fmt::Formatter::write_str(f, "Value55"),
            (&Variant::ValueAA,) => ::core::fmt::Formatter::write_str(f, "ValueAA"),
        }
    }
}
impl ::core::marker::StructuralPartialEq for Variant {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Variant {
    #[inline]
    fn eq(&self, other: &Variant) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => true,
                }
            } else {
                false
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Variant {
    #[inline]
    fn clone(&self) -> Variant {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for Variant {}
impl Default for Variant {
    fn default() -> Self {
        Variant::Value55
    }
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
struct ControlPacket {
    #[bitfield(8)]
    id: u8,
    #[bitfield(4)]
    dataType1: u8,
    #[bitfield(4)]
    dataType2: u8,
    # [bitfield (16 , name = data_len , value = self . data . len () , pack = LE , unpack = LE)]
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
        let mut pack_le = false;
        dst[offset] |= (value as u8) & 255u8;
        offset += 1;
        if 1usize + offset > dst.len() {
            return Err(bitwrap_extra::BitWrapError);
        }
        dst[offset] = 0;
        let value: u8 = u8::try_from(self.dataType1)?;
        let mut pack_le = false;
        dst[offset] |= ((value as u8) & 15u8) << 4usize;
        let value: u8 = u8::try_from(self.dataType2)?;
        let mut pack_le = false;
        dst[offset] |= (value as u8) & 15u8;
        offset += 1;
        if 2usize + offset > dst.len() {
            return Err(bitwrap_extra::BitWrapError);
        }
        dst[offset] = 0;
        let value = (self.data.len()) as u16;
        let data_len = value;
        let mut pack_le = true;
        dst[offset] |= ((value >> 0usize) as u8) & 255u8;
        offset += 1;
        dst[offset] = 0;
        dst[offset] |= ((value >> 8usize) as u8) & 255u8;
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
        let mut pack_le = false;
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
        let mut unpack_le = false;
        value |= (src[offset] & 255u8) as u8;
        offset += 1;
        self.id = u8::try_from(value)?;
        if 1usize + offset > src.len() {
            return Err(bitwrap_extra::BitWrapError);
        }
        let mut value: u8 = 0;
        let mut unpack_le = false;
        value |= ((src[offset] >> 4usize) & 15u8) as u8;
        self.dataType1 = u8::try_from(value)?;
        let mut value: u8 = 0;
        let mut unpack_le = false;
        value |= (src[offset] & 15u8) as u8;
        offset += 1;
        self.dataType2 = u8::try_from(value)?;
        if 2usize + offset > src.len() {
            return Err(bitwrap_extra::BitWrapError);
        }
        let mut value: u16 = 0;
        let mut unpack_le = true;
        value |= ((src[offset] & 255u8) as u16) << 0usize;
        offset += 1;
        value |= ((src[offset] & 255u8) as u16) << 8usize;
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
        let mut unpack_le = false;
        value |= (src[offset] & 255u8) as u8;
        offset += 1;
        self.crc = u8::try_from(value)?;
        Ok(offset)
    }
}
fn main() {
    let mut packet = ControlPacket::default();
    packet.id = 1;
    packet.dataType1 = 2;
    packet.dataType2 = 2;
    packet.data = <[_]>::into_vec(box [1, 2, 3]);
    packet.crc = 2;
    let len: usize = 8;
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
    const DATA: &[u8] = &[1, 2, 3, 0, 1, 2, 3, 2];
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
    let test = 3 as u16;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["res ", "\n"],
            &match (&test.to_be_bytes(),) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    let val = test.to_le_bytes();
    let (int_bytes, rest) = val.split_at(std::mem::size_of::<u16>());
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["res ", "\n"],
            &match (&rest,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["res ", "\n"],
            &match (&u16::from_le_bytes(val[..].try_into().unwrap()),) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    for i in val {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["val: ", "\n"],
                &match (&i,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
        }
    }
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["res ", "\n"],
            &match (&test.to_le_bytes(),) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["res ", "\n"],
            &match (&packet.data_len,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
}
