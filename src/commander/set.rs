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

use super::{
    Commander,
    ParseErrorPrefixise
};

impl Commander {
    pub fn set(&mut self, paramstr: &[&str]) -> Result<(), String> {
        if paramstr.len() >= 2 {
            let setting = paramstr[0].to_lowercase();
            let value = paramstr[1];
            match setting.as_str() {
                "show_abs_time" => {
                    match value.parse::<bool>() {
                        Ok(b) => {
                            self.settings.show_abs_time = b;
                            println!("{}", "Set".green());
                            Ok(())
                        },
                        Err(err) => err.prefixised("bool value")
                    }
                },
                "show_ticks" => {
                    match value.parse::<bool>() {
                        Ok(b) => {
                            self.settings.show_ticks = b;
                            println!("{}", "Set".green());
                            Ok(())
                        },
                        Err(err) => err.prefixised("bool value")
                    }
                },
                _ => {
                    Err(String::from("Unknown setting"))
                }
            }
        } else if paramstr.len() == 1 {
            Err(String::from("Value not specified"))
        } else {
            Err(String::from("Setting not specified"))
        }
    }

}