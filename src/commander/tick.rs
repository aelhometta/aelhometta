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

use std::io::{
    self,
    Write
};

use {
    crate::aelhometta::{
        Uid,
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
    pub fn tick(&mut self, æh: &mut Ælhometta, paramstr: &[&str]) -> Result<(), String> {
        let (couid, count) = if paramstr.len() >= 1 {
            match paramstr[0].parse_hex::<Uid>() {
                Ok(uid) => {
                    if paramstr.len() >= 2 {
                        match paramstr[1].parse::<usize>() {
                            Ok(c) => {
                                if c > 0 {
                                    (Some(uid), c)
                                } else {
                                    return Err(String::from("Count must be greater than 0"));
                                }                                                        
                            },
                            Err(err) => {
                                return err.prefixised("count");
                            }
                        }
                    } else {
                        (Some(uid), 1)
                    }
                }
                Err(err) => {
                    return err.prefixised("ctrl uid")
                }
            }
        } else {
            (None, 1)
        };

        for _ in 0..count {
            let tick_data = æh.tick(&couid);
            println!("{}{}{}{}{}{}{}{}{}{}{}",
                "Age ".dark_blue(),
                format!("{}", æh.age()).blue(),
                " : ".dark_grey(),
                "Ctrl ".dark_cyan(),
                tick_data.controller_optuid.hexly().cyan(),
                " | ".dark_grey(),
                "Node ".dark_magenta(),
                tick_data.exec_optuid.hexly().magenta(),
                " | ".dark_grey(),
                "Cont ".dark_yellow(),
                match tick_data.exec_optcontent {
                    Some(content) => format!("{:?}", content),
                    None => format!("×")
                }.yellow()
            );
            io::stdout().flush().unwrap_or(());
            if couid.is_some() && (tick_data.exec_optcontent == None) {
                break;
            }
        }

        Ok(())
    }

}