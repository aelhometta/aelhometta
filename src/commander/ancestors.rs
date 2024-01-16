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

use crossterm::style::Stylize;

use {
    crate::aelhometta::Ælhometta,
    super::{
        Commander,
        ParseErrorPrefixise
    }
};

impl Commander {
    pub fn ancestors(&self, æh: &mut Ælhometta, paramstr: &[&str]) -> Result<(), String> {
        if paramstr.len() > 0 {
            let kin = paramstr[0].to_lowercase();
            match kin.as_str() {
                "a" => {
                    if paramstr.len() > 1 {
                        match paramstr[1].parse::<u8>() {
                            Ok(sterile_power) => {
                                if paramstr.len() > 2 {
                                    match paramstr[2].parse::<u8>() {
                                        Ok(skip_power) => {
                                            if paramstr.len() > 3 {
                                                match paramstr[3].parse::<usize>() {
                                                    Ok(spacity) => {
                                                        æh.introduce_ancestor_a(sterile_power, skip_power, spacity);
                                                        println!("{}", "Introduced".green());
                                                        Ok(())
                                                    },
                                                    Err(err) => err.prefixised("spacity")
                                                }
                                            } else {
                                                Err(format!("Spacity not specified"))
                                            }
                                        },
                                        Err(err) => err.prefixised("skip-power")
                                    }
                                } else {
                                    Err(format!("Skip-power not specified"))
                                }
                            },
                            Err(err) => err.prefixised("sterile-power")
                        }
                    } else {
                        Err(format!("Sterile-power not specified"))
                    }
                },

                "b" => {
                    if paramstr.len() > 1 {
                        match paramstr[1].parse::<usize>() {
                            Ok(spacity) => {
                                æh.introduce_ancestor_b(spacity);
                                println!("{}", "Introduced".green());
                                Ok(())
                            },
                            Err(err) => err.prefixised("spacity")
                        }
                    } else {
                        Err(format!("Spacity not specified"))
                    }
                }

                _ => Err(format!("Unknown kin '{}'", kin))
                }
        } else {
            Err(String::from("Kin not specified"))
        }
    }

}