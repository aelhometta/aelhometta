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

use super::Commander;

impl Commander {
    fn print_show_ticks(&self) {
        println!("{}{}", format!("{:16}", "show_ticks").dark_yellow(), format!("{}", self.settings.show_ticks).dark_yellow().bold());
    }

    fn print_show_abs_time(&self) {
        println!("{}{}", format!("{:16}", "show_abs_time").dark_yellow(), format!("{}", self.settings.show_abs_time).dark_yellow().bold());
    }

    pub fn settings(&self, setting: &str) -> Result<(), String> {
        match setting {
            "" => {
                self.print_show_abs_time();
                self.print_show_ticks();
                Ok(())
            },
            "show_abs_time" => {
                self.print_show_abs_time();
                Ok(())
            }
            "show_ticks" => {
                self.print_show_ticks();
                Ok(())
            },
            _ => {
                Err(format!("Unknown setting"))
            }
        }
    }

}