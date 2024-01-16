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
    crate::{
        aelhometta::{
            Ælhometta,
            Content,
            Hexly,
            Optuid,
            Uid
        },
        serbin::OtBits
    },
    super::{
        Commander,
        ParseErrorPrefixise,
        ParseHex
    }
};

impl Commander {
    pub fn shownode(&self, æh: &Ælhometta, paramstr: &[&str]) -> Result<(), String> {
        if paramstr.len() > 0 {
            match paramstr[0].parse_hex::<Uid>() {
                Ok(nuid) => {
                    match æh.node(&nuid) {
                        Some(node) => {
                            println!("{}{}", format!("{:16}", "Content").dark_yellow(), format!("{:?}", Content::ot_bits(node.b_content())).yellow());
                            println!("{}{}", format!("{:16}", "Next").dark_magenta(), Optuid::ot_bits(node.b_next()).hexly().magenta());
                            println!("{}{}", format!("{:16}", "AltNext").dark_magenta(), Optuid::ot_bits(node.b_altnext()).hexly().magenta());
                            Ok(())
                        },
                        None => Err(format!("Uid not found"))
                    }
                },
                Err(err) => err.prefixised("node uid")
            }
        } else {
            Err(String::from("Uid not specified"))
        }
    }

}