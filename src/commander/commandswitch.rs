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

use num_traits::FromPrimitive;

use {
    crate::aelhometta::{
        Ælhometta,
        Command
    },
    super::{
        Commander,
        ParseErrorPrefixise,
        ParseHex
    }
};

impl Commander {
    pub fn commandswitch(&mut self, æh: &mut Ælhometta, paramstr: &[&str]) -> Result<(), String> {
        if paramstr.len() > 0 {
            match paramstr[0].parse_hex::<u8>() {
                Ok(cind) => {
                    if cind < 0x80 {
                        let cind = cind as u8;
                        if let Some(comm) = Command::from_u8(cind) {
                            æh.change_commandswitch(cind);
                            println!("{} {} {} {}",
                                "Changed".green(),
                                format!("{:?}", comm).yellow(),
                                "to".green(),
                                format!("{}", æh.commandswitch(cind)).dark_yellow().bold()
                            );
                            Ok(())        
                        } else {
                            Err(format!("There is no command with index {}", cind))
                        }
                    } else {
                        Err(String::from("Command index must be not greater than 127"))
                    }
                },
                Err(err) => err.prefixised("command index")
            }
        } else {
            for cind in 0..0x80u8 {
                if let Some(comm) = Command::from_u8(cind) {
                    println!("{}{}{}", 
                        format!("{:<4}", format!("{:02X}", cind)).blue(),
                        format!("{:<32}", format!("{:?}", comm)).yellow(),
                        format!("{}", æh.commandswitch(cind)).dark_yellow().bold()
                    );
                }
            }
            Ok(())
        }
    }

}