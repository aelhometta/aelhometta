/*
 * Ælhometta
 *
 * Archaic attempt at autonomous non-sandboxed distributed artificial life
 * of assembler automaton type, it features: separation of descriptive and
 * executive data that provides branches and loops without jump instructions,
 * encrypted publish-subscribe interaction with other instances over Tor,
 * input/output through ordinary files associated with external sensors and
 * actuators, and built-in shell.
 * 
 * https://github.com/aelhometta/aelhometta
 * 
 * aelhometta@proton.me
 * 
 * Copyright (c) 2024 Ælhometta shapers
 * 
 * Ælhometta is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Ælhometta is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ælhometta. If not, see <https://www.gnu.org/licenses/>.
 */

use std::io::{
    Read,
    Write
};

// TODO: find a crate that already does all the following and more

pub trait WriteBin<T> {
    fn write_bin(&mut self, x: T) -> Result<(), String>;
}

pub trait ReadBin<T> {
    fn read_bin(&mut self) -> Result<T, String>;
}

pub trait OtBits<T> {
    fn ot_bits(x: T) -> Self;
}

pub trait ToBits<T> {
    fn to_bits(&self) -> T;
}

// u128 (largest unsigned int)
// using LEB128 encoding

impl<W: Write> WriteBin<u128> for W {
    fn write_bin(&mut self, mut x: u128) -> Result<(), String> {
        let mut part: u8;
        loop {
            part = (x & 0x7F) as u8;
            if x >= 0x80 {
                part |= 0x80; // Bit 7 is set iff more parts follow
            }
            self.write_all(&[part]).map_err(|e| e.to_string())?;
            x >>= 7;
            if x == 0 {
                break;
            }
        }
        Ok(())
    }
}

impl<R: Read> ReadBin<u128> for R {
    fn read_bin(&mut self) -> Result<u128, String> {
        let mut shift: u32 = 0;
        let mut x: u128 = 0;
        let mut buf8 = [0u8; 1];
        loop {
            self.read_exact(&mut buf8).map_err(|e| e.to_string())?;
            x += ((buf8[0] & 0x7F) as u128) << shift;
            if (buf8[0] & 0x80) == 0 { // Bit 7 is not set iff no more parts follow
                break;
            }
            shift += 7;
        }
        Ok(x)
    }
}

// i128

impl<W: Write> WriteBin<i128> for W {
    fn write_bin(&mut self, x: i128) -> Result<(), String> {
        self.write_bin(x as u128)
    }
}

impl<R: Read> ReadBin<i128> for R {
    fn read_bin(&mut self) -> Result<i128, String> {
        let x: u128 = self.read_bin()?;
        Ok(x as i128)
    }
}

// u64 & i64

impl<W: Write> WriteBin<u64> for W {
    fn write_bin(&mut self, x: u64) -> Result<(), String> {
        self.write_bin(x as u128)
    }
}

impl<R: Read> ReadBin<u64> for R {
    fn read_bin(&mut self) -> Result<u64, String> {
        let x: u128 = self.read_bin()?;
        Ok(x as u64)
    }
}

impl<W: Write> WriteBin<i64> for W {
    fn write_bin(&mut self, x: i64) -> Result<(), String> {
        self.write_bin(x as u64)
    }
}

impl<R: Read> ReadBin<i64> for R {
    fn read_bin(&mut self) -> Result<i64, String> {
        let x: u64 = self.read_bin()?;
        Ok(x as i64)
    }
}

// u32 & i32

impl<W: Write> WriteBin<u32> for W {
    fn write_bin(&mut self, x: u32) -> Result<(), String> {
        self.write_bin(x as u128)
    }
}

impl<R: Read> ReadBin<u32> for R {
    fn read_bin(&mut self) -> Result<u32, String> {
       let x: u128 = self.read_bin()?;
       Ok(x as u32)
    }
}

impl<W: Write> WriteBin<i32> for W {
    fn write_bin(&mut self, x: i32) -> Result<(), String> {
        self.write_bin(x as u32)
    }
}

impl<R: Read> ReadBin<i32> for R {
    fn read_bin(&mut self) -> Result<i32, String> {
        let x: u32 = self.read_bin()?;
        Ok(x as i32)
    }
}

// u16 & i16

impl<W: Write> WriteBin<u16> for W {
    fn write_bin(&mut self, x: u16) -> Result<(), String> {
        self.write_bin(x as u128)
    }
}

impl<R: Read> ReadBin<u16> for R {
    fn read_bin(&mut self) -> Result<u16, String> {
        let x: u128 = self.read_bin()?;
        Ok(x as u16)
    }
}

impl<W: Write> WriteBin<i16> for W {
    fn write_bin(&mut self, x: i16) -> Result<(), String> {
        self.write_bin(x as u16)
    }
}

impl<R: Read> ReadBin<i16> for R {
    fn read_bin(&mut self) -> Result<i16, String> {
        let x: u16 = self.read_bin()?;
        Ok(x as i16)
    }
}

// u8 & i8
// There is no LEB128 compression for these, so (de)serialize "verbatim"

impl<W: Write> WriteBin<u8> for W {
    fn write_bin(&mut self, x: u8) -> Result<(), String> {
        self.write_all(& x.to_le_bytes()).map_err(|e| e.to_string())
    }
}

impl<R: Read> ReadBin<u8> for R {
    fn read_bin(&mut self) -> Result<u8, String> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf).map_err(|e| e.to_string())?;
        Ok(u8::from_le_bytes(buf))
    }
}

impl<W: Write> WriteBin<i8> for W {
    fn write_bin(&mut self, x: i8) -> Result<(), String> {
        self.write_bin(x as u8)
    }
}

impl<R: Read> ReadBin<i8> for R {
    fn read_bin(&mut self) -> Result<i8, String> {
        let x: u8 = self.read_bin()?;
        Ok(x as i8)
    }
}

// bool

impl<W: Write> WriteBin<bool> for W {
    fn write_bin(&mut self, x: bool) -> Result<(), String> {
        self.write_bin(x as u8)
    }
}

impl<R: Read> ReadBin<bool> for R {
    fn read_bin(&mut self) -> Result<bool, String> {
        let b: u8 = self.read_bin()?;
        Ok(b != 0)
    }
}

// usize & isize

impl<W: Write> WriteBin<usize> for W {
    fn write_bin(&mut self, x: usize) -> Result<(), String> {
        self.write_bin(x as u128)
    }
}

impl<R: Read> ReadBin<usize> for R {
    fn read_bin(&mut self) -> Result<usize, String> {
        let x: u128 = self.read_bin()?;
        Ok(x as usize)
    }
}

impl<W: Write> WriteBin<isize> for W {
    fn write_bin(&mut self, x: isize) -> Result<(), String> {
        self.write_bin(x as usize)
    }
}

impl<R: Read> ReadBin<isize> for R {
    fn read_bin(&mut self) -> Result<isize, String> {
        let x: usize = self.read_bin()?;
        Ok(x as isize)
    }
}

// f64

impl<W: Write> WriteBin<f64> for W {
    fn write_bin(&mut self, x: f64) -> Result<(), String> {
        self.write_all(& x.to_le_bytes()).map_err(|e| e.to_string())
    }
}

impl<R: Read> ReadBin<f64> for R {
    fn read_bin(&mut self) -> Result<f64, String> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf).map_err(|e| e.to_string())?;
        Ok(f64::from_le_bytes(buf))
    }
}

// String

impl<W: Write> WriteBin<&str> for W {
    fn write_bin(&mut self, x: &str) -> Result<(), String> {
        self.write_bin(x.len())?;
        self.write_all(x.as_bytes()).map_err(|e| e.to_string())
    }
}

impl<R: Read> ReadBin<String> for R {
    fn read_bin(&mut self) -> Result<String, String> {
        let l: usize = self.read_bin()?;
        let mut strbuf = vec![0u8; l];
        self.read_exact(&mut strbuf).map_err(|e| e.to_string())?;
        String::from_utf8(strbuf).map_err(|e| e.to_string())
    }
}

// Some composites...

impl<W: Write + WriteBin<usize> + WriteBin<WB>, WB> WriteBin<Option<WB>> for W {
    fn write_bin(&mut self, opt: Option<WB>) -> Result<(), String> {
        match opt {
            Some(x) => {
                self.write_bin(true)?;
                self.write_bin(x)?;
            },
            None => {
                self.write_bin(false)?;
            }
        }
        Ok(())
    }
}

impl<R: Read + ReadBin<usize> + ReadBin<RB>, RB> ReadBin<Option<RB>> for R {
    fn read_bin(&mut self) -> Result<Option<RB>, String> {
        let is_some: bool = self.read_bin()?;
        match is_some {
            true => {
                Ok(Some(self.read_bin()?))
            },
            false => Ok(None)
        }
    }
}
