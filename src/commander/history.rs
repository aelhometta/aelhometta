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

use chrono::prelude::*;

use crossterm::style::Stylize;

use super::{
    HISTORY_MAX_LEN,
    Commander,
    ParseErrorPrefixise
};

impl Commander {
    pub fn history(&self, paramstr: &[&str]) -> Result<(), String> {
        let limit = if paramstr.len() >= 1 {
            match paramstr[0].parse::<usize>() {
                Ok(lim) => {
                    if lim > 0 {
                        lim
                    } else {
                        return Err(String::from("Limit must be greater than 0"));
                    }                                                        
                },
                Err(err) => {
                    return err.prefixised("limit");
                }
            }
        } else {
            HISTORY_MAX_LEN
        };

        let past = self.history.past(limit);
        for record in past {
            let (command, timestamp) = (record.command(), record.timestamp());
            println!("{} {}",
                format!("{}.{:03} UTC", NaiveDateTime::from_timestamp_micros(timestamp).unwrap_or_default().format("%Y.%m.%d %a %H:%M:%S"), (timestamp / 1000) % 1000).dark_green(),
                format!("@ {}", command).dark_grey()
            );
        }
        Ok(())
    }

}