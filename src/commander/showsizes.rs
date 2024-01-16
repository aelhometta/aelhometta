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
        HISTORY_MAX_LEN,
        Commander
    }
};

impl Commander {
    pub fn showsizes(&self, æh: &Ælhometta) -> Result<(), String> {
        println!("{:8}{}", " ", "Ælhometta".dark_grey());

        println!("{}{}",
            format!("{:24}", "Optuid channels").dark_magenta(),
            format!("{}", æh.ether_optuids().len()).magenta() 
        );
        println!("{}{}",
            format!("{:24}", "Integer channels").dark_blue(),
            format!("{}", æh.ether_integers().len()).blue()
        );

        println!("{:8}{}", " ", "Commander".dark_grey());
        
        println!("{}{}",
            format!("{:24}", "Max history length").dark_grey(),
            format!("{}", HISTORY_MAX_LEN).dark_grey().bold()
        );        

        Ok(())
    }

}