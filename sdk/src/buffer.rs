// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;
use std::convert::TryInto;

use iota_sdk::U256;

use crate::error::{Error, Result};

#[derive(Debug, Default, Clone)]
pub struct SimpleBufferCursor {
    buffer: Vec<u8>,
    traverse: usize,
}

impl SimpleBufferCursor {
    pub fn from(buffer: Vec<u8>) -> Self {
        SimpleBufferCursor { buffer, traverse: 0 }
    }

    pub fn next(&mut self) -> Result<u8> {
        if self.traverse >= self.buffer.len() {
            return Err(Error::IO {
                expected: std::io::ErrorKind::Other,
                message: "empty buffer",
            });
        }

        let ret = self.buffer[self.traverse];
        self.traverse += 1;
        Ok(ret)
    }

    pub fn read_int_be(&mut self, length: usize) -> i32 {
        let value_bytes: &[u8] = &self.buffer[self.traverse..self.traverse + length];
        self.traverse += length;
        i32::from_be_bytes(value_bytes.try_into().unwrap())
    }

    pub fn read_uint32_le(&mut self) -> u32 {
        let value_bytes: &[u8] = &self.buffer[self.traverse..self.traverse + 4];
        self.traverse += 4;
        u32::from_le_bytes(value_bytes.try_into().unwrap())
    }

    pub fn read_uint64_le(&mut self) -> u64 {
        let value_bytes: &[u8] = &self.buffer[self.traverse..self.traverse + 8];
        self.traverse += 8;
        u64::from_le_bytes(value_bytes.try_into().unwrap())
    }

    pub fn read_uint16_le(&mut self) -> u16 {
        let value_bytes: &[u8] = &self.buffer[self.traverse..self.traverse + 2];
        self.traverse += 2;
        u16::from_le_bytes(value_bytes.try_into().unwrap())
    }

    pub fn read_u256_be(&mut self) -> Result<U256> {
        let size = self.next()?;
        println!("{}", size);
        let mut bytes = vec![];
        for _ in 0..size {
            bytes.push(self.next()?)
        }

        Ok(U256::from_big_endian(&bytes))
    }

    pub fn read_uint64_special_encoding(&mut self) -> Result<u64> {
        self.size64_decode()
    }

    pub fn read_uint32_special_encoding(&mut self) -> Result<u32> {
        let num = self.size64_decode()?;
        if num > u32::MAX as u64 {
            return Err(Error::IO {
                expected: std::io::ErrorKind::Other,
                message: "found a u64 when expecting u32 max",
            });
        }
        Ok(num as u32)
    }

    pub fn read_bytes(&mut self, length: usize) -> Vec<u8> {
        let sub_buffer: Vec<u8> = self.buffer[self.traverse..self.traverse + length].to_vec();
        self.traverse += length;
        sub_buffer
    }

    pub fn write_int_be(&mut self, value: i32, length: usize) {
        let mut n_buffer = value.to_be_bytes().to_vec();
        n_buffer.truncate(length);
        self.buffer.extend(&n_buffer);
    }

    pub fn write_int8(&mut self, value: i8) {
        self.buffer.push(value as u8);
    }

    pub fn write_uint8(&mut self, value: u8) {
        self.buffer.push(value);
    }

    pub fn write_uint32_le(&mut self, value: u32) {
        self.buffer.extend(&value.to_le_bytes());
    }

    pub fn write_uint16_le(&mut self, value: u16) {
        self.buffer.extend(&value.to_le_bytes());
    }

    pub fn write_uint8_array(&mut self, bytes: &[u8]) {
        self.buffer.extend(bytes);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    pub fn write_u256_be(&mut self, value: U256) {
        let bytes: [u8; 32] = value.into();
        let first_non_zero_index = bytes.iter().position(|&x| x != 0).unwrap_or(32);
        self.write_uint8(32 - (first_non_zero_index as u8));
        self.buffer.extend_from_slice(&bytes[first_non_zero_index..]);
    }

    pub fn write_uint64_special_encoding(&mut self, value: u64) {
        self.write_bytes(&size64_encode(value));
    }

    pub fn write_uint32_special_encoding(&mut self, value: u32) {
        self.write_bytes(&size64_encode(u64::from(value)));
    }

    pub fn serialize(&self) -> String {
        return hex::encode(&self.buffer);
    }

    // size64_decode uses a simple variable length encoding scheme
    // It takes groups of 7 bits per byte, and decodes following groups while
    // the 0x80 bit is set. Since most numbers are small, this will result in
    // significant storage savings, with values < 128 occupying only a single
    // byte, and values < 16384 only 2 bytes.
    pub fn size64_decode(&mut self) -> Result<u64> {
        let mut value: u64 = 0;
        for shift in (0..64).step_by(7) {
            let mut byte = self.next()?;
            let is_last_byte = byte & 0x80 == 0;
            byte &= 0x7F;
            value |= (byte as u64) << shift;
            if is_last_byte {
                return Ok(value);
            }
        }
        Err(Error::IO {
            expected: std::io::ErrorKind::Other,
            message: "size64 overflow",
        })
    }
}

impl Deref for SimpleBufferCursor {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

/// size64_encode uses a simple variable length encoding scheme
/// It takes groups of 7 bits per byte, and encodes if there will be a next group
/// by setting the 0x80 bit. Since most numbers are small, this will result in
/// significant storage savings, with values < 128 occupying only a single byte,
/// and values < 16384 only 2 bytes.
fn size64_encode(mut s: u64) -> Vec<u8> {
    let mut result = Vec::new();
    loop {
        let mut byte = (s & 0x7F) as u8;
        s >>= 7;
        if s != 0 {
            byte |= 0x80;
        }
        result.push(byte);
        if s == 0 {
            break;
        }
    }
    result
}