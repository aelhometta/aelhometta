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
    crate::aelhometta::{
        Ælhometta,
        Hexly
    },
    super::{
        Commander,
        ParseErrorPrefixise,
        ParseHex
    }
};

impl Commander {
    pub fn random(&self, æh: &mut Ælhometta, paramstr: &[&str]) -> Result<(), String> {
        if paramstr.len() > 0 {
            let entitype = paramstr[0].to_lowercase();
            match entitype.as_str() {
                "ctrl" => {
                    println!("{}", æh.random_controller_optuid().hexly().cyan());
                    Ok(())
                },

                "node" => {
                    if paramstr.len() > 1 {
                        match paramstr[1].parse_hex::<u8>() {
                            Ok(b_content) => {
                                if b_content < 0x100 {
                                    let b_content = b_content as u8;
                                    println!("{}", æh.random_node_with_bcontent_optuid(b_content).hexly().magenta());
                                    Ok(())
                                } else {
                                    Err(String::from("Byte content must be not greater than 255"))
                                }
                            },
                            Err(err) => err.prefixised("byte content")
                        }
                    } else {
                        println!("{}", æh.random_node_optuid().hexly().magenta());
                        Ok(())
                    }                    
                },

                _ => {
                    Err(format!("Unknown or unsupported entity type '{}'", &entitype))
                }
            }
        } else {
            Err(String::from("Entity type not specified"))
        }
    }

}