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

use std::{
    fs,
    time::{
        SystemTime,
        UNIX_EPOCH
    }
};

use super::{
    IntegersFileMapping,
    Ælhometta
};

impl Ælhometta {
    pub fn iomap_update(&mut self) {
        let ut = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |dur| dur.as_micros()) as i64;

        for om in &mut self.output_mappings {
            if ut - om.ut_last_update > om.interval {
                let buf = self.ether_integers[om.start..(om.start + om.length)].iter().map(|i| 
                    i.to_le_bytes()).flatten().collect::<Vec<u8>>();
                let _ = fs::write(& om.filepath, &buf);
                om.ut_last_update = ut;
            }
        }

        for im in &mut self.input_mappings {
            if ut - im.ut_last_update > im.interval {
                if let Ok(bufall) = fs::read(& im.filepath) {
                    if bufall.len() >= (im.length << 3) {
                        let mut bufone = [0u8; 8];
                        for i in 0..im.length {
                            bufone.copy_from_slice(& bufall[(i << 3)..((i + 1) << 3)]);
                            self.ether_integers[im.start + i] = i64::from_le_bytes(bufone);
                        }
                    }
                }
                im.ut_last_update = ut;
            }
        }
    }

    pub fn iomap_out_add(&mut self, start: usize, length: usize, interval: i64, filepath: &str) -> Result<(), String> {
        if length > 0 {
            if start + length <= self.ether_integers.len() {
                if interval > 0 {
                    self.output_mappings.push(IntegersFileMapping::new(
                        start, length, interval, filepath
                    ));
                    Ok(())
                } else {
                    Err(String::from("Interval must be greater than 0"))
                }
            } else {
                Err(format!("Mapping ends at {}, but there are only {} integer channels", start + length, self.ether_integers.len()))
            }        
        } else {
            Err(String::from("Length must be greater than 0"))
        }
    }

    pub fn iomap_out_del(&mut self, index: usize) -> Result<(), String> {
        if index < self.output_mappings.len() {
            self.output_mappings.remove(index);
            Ok(())
        } else {
            Err(format!("There are only {} output mappings", self.output_mappings.len()))
        }
    }

    pub fn iomap_in_add(&mut self, start: usize, length: usize, interval: i64, filepath: &str) -> Result<(), String> {
        if length > 0 {
            if start + length <= self.ether_integers.len() {
                if interval > 0 {
                    self.input_mappings.push(IntegersFileMapping::new(
                        start, length, interval, filepath
                    ));
                    Ok(())
                } else {
                    Err(String::from("Interval must be greater than 0"))
                }
            } else {
                Err(format!("Mapping ends at {}, but there are only {} integer channels", start + length, self.ether_integers.len()))
            }        
        } else {
            Err(String::from("Length must be greater than 0"))
        }
    }

    pub fn iomap_in_del(&mut self, index: usize) -> Result<(), String> {
        if index < self.input_mappings.len() {
            self.input_mappings.remove(index);
            Ok(())
        } else {
            Err(format!("There are only {} input mappings", self.input_mappings.len()))
        }
    }
}