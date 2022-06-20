//! # bitwrap_extra
//!
//! [![docs](https://docs.rs/bitwrap/badge.svg)](https://docs.rs/bitwrap_extra)
//!
//! ## Intro
//!
//! bitwrap is a derive macro and interface to declare a struct data member
//! with explicit size, in bits.

#![cfg_attr(not(feature = "std"), no_std)]


use {
    core::{
        fmt,
        convert::Infallible,
    },
};


pub use {
    bitwrap_derive_extra::{BitWrap},
};


#[derive(Debug, PartialEq)]
pub struct BitWrapError;


impl fmt::Display for BitWrapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "index out of bounds")
    }
}


#[cfg(feature = "std")]
impl std::error::Error for BitWrapError {}


impl From<Infallible> for BitWrapError {
    fn from(x: Infallible) -> BitWrapError {
        match x {}
    }
}


pub trait BitWrapExt {
    /// Build byte array
    fn pack(&self) -> Result<Vec<u8>, BitWrapError>;

    /// Extract object field values from byte array
    fn unpack(&mut self, src: &[u8]) -> Result<usize, BitWrapError>;

    /// len
    fn len(&self) -> usize;
}


#[cfg(feature = "std")]
impl BitWrapExt for Vec<u8> {
    #[inline]
    fn pack(&self) -> Result<Vec<u8>, BitWrapError> {
        let len = self.len() as usize;
        let mut dst = vec![0 as u8; len];
        dst[.. len].clone_from_slice(self.as_slice());
        Ok(dst)
    }

    #[inline]
    fn unpack(&mut self, src: &[u8]) -> Result<usize, BitWrapError> {
        self.extend_from_slice(src);
        Ok(src.len())
    }

    fn len(&self) -> usize {
        self.len()
    }
}


#[cfg(feature = "std")]
impl<T: BitWrapExt + Default> BitWrapExt for Vec<T> {
    #[inline]
    fn pack(&self) -> Result<Vec<u8>, BitWrapError> {
        let len = self.len() as usize;
        let mut dst = vec![0 as u8; len];
        let mut skip = 0;
        for item in self {
            let tmp = item.pack()?;
            let tmp_len = tmp.len();
            dst[skip..skip+ tmp_len].clone_from_slice(tmp.as_slice());
            skip += tmp_len;
        }
        Ok(dst)
    }

    #[inline]
    fn unpack(&mut self, src: &[u8]) -> Result<usize, BitWrapError> {
        let mut skip = 0;
        while skip < src.len() {
            let mut item = T::default();
            skip += item.unpack(&src[skip ..])?;
            self.push(item);
        }
        Ok(skip)
    }

    fn len(&self) -> usize {
        self.len()
    }
}
