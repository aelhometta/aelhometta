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
        ParseErrorPrefixise
    }
};

impl Commander {
    pub fn ether(&self, æh: &Ælhometta, paramstr: &[&str]) -> Result<(), String> {
        if paramstr.len() >= 1 {
            let chantype = paramstr[0].to_lowercase();
            if paramstr.len() >= 2 {
                match paramstr[1].parse::<usize>() {
                    Ok(start) => {
                        let length: usize = if paramstr.len() >= 3 {
                            match paramstr[2].parse::<usize>() {
                                Ok(l) => {
                                    if l > 0 {
                                        l
                                    } else {
                                        return Err(String::from("Length must be greater than 0"));
                                    }
                                },
                                Err(err) => {
                                    return err.prefixised("range length");
                                }
                            }
                        } else {
                            1
                        };
                        
                        match chantype.as_str() {
                            "ouid" | "optuid" => {
                                let ether_optuids = æh.ether_optuids();
                                for chan in start..(start + length) {
                                    let chan_content_str_styl = if chan < ether_optuids.len() {
                                        ether_optuids[chan].hexly().magenta()
                                    } else {
                                        String::from("OUT OF BOUNDS").red()
                                    };
                                    println!("{}{:8}{}", format!("{:>12}", chan).dark_magenta(), " ", chan_content_str_styl);
                                }
                                Ok(())
                            },
                            "int" | "integer" => {
                                let ether_integers = æh.ether_integers();
                                for chan in start..(start + length) {
                                    if chan < ether_integers.len() {
                                        println!("{}{:8}{}", format!("{:>12}", chan).dark_blue(), " ", format!("{0}={0:X}h", ether_integers[chan]).blue());
                                    } else {
                                        println!("{}{:8}{}", format!("{:>12}", chan).dark_blue(), " ", "OUT OF BOUNDS".red());
                                    };                                    
                                }
                                Ok(())
                            },
                            _ => {
                                Err(format!("Unknown channels type '{}'", chantype))
                            }
                        }  
                    },
                    Err(err) => {
                        err.prefixised("range start")
                    }
                }
            } else {
                return Err(String::from("Range start not specified"));
            }
        } else {
            Err(String::from("Channels type not specified"))
        }
    }

}