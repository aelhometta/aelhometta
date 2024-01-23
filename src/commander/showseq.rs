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

use std::collections::HashSet;

use crate::aelhometta::Hexly;

use {
    crate::{
        aelhometta::{
            Ælhometta,
            Content,
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
    pub fn showseq(&self, æh: &Ælhometta, paramstr: &[&str]) -> Result<(), String> {
        let (snuid, limit) = if paramstr.len() >= 1 {
            match paramstr[0].parse_hex::<Uid>() {
                Ok(uid) => {
                    if paramstr.len() >= 2 {
                        match paramstr[1].parse::<usize>() {
                            Ok(lim) => {
                                if lim > 0 {
                                    (uid, lim)
                                } else {
                                    return Err(String::from("Limit must be greater than 0"));
                                }                                                        
                            }
                            Err(err) => {
                                return err.prefixised("limit");
                            }
                        }
                    } else {
                        (uid, self.settings.sequence_def_limit)
                    }
                },
                Err(err) => {
                    return err.prefixised("start node uid");
                }
            }
        } else {
            return Err(String::from("Uid not specified"))
        };

        match æh.node(&snuid) {
            Some(snode) => {
                let mut nuid = snuid;
                let mut node = snode;
                let mut nuids_set: HashSet<Uid> = HashSet::new();
                let mut n: usize = 0;
                loop {
                    println!("{}{}", format!("{:20}", Some(nuid).hexly()).magenta(), format!("{:?}", Content::ot_bits(node.b_content())).yellow());
                    n += 1;
                    if ! nuids_set.insert(nuid) {
                        println!("{}", "LOOP".dark_yellow().bold());
                        break;
                    };
                    if n >= limit {
                        println!("{}", "LIMIT".dark_yellow().bold());
                        break;
                    }
                    match Optuid::ot_bits(node.b_next()) {
                        Some(nnuid) => {
                            nuid = nnuid;
                            match æh.node(&nuid) {
                                Some(nnode) => {
                                    node = nnode;
                                },
                                None => {
                                    println!("{}", "Next uid not found".red().bold());
                                }
                            }
                        },
                        None => {
                            println!("{}", "×".magenta());
                            break;
                        }
                    }
                }
                Ok(())
            },
            None => Err(format!("Uid not found"))
        }
    }

}