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

use std::{
    collections::HashSet,
    io::{
        self,
        Write
    }
};

use crate::aelhometta::Hexly;

use {
    crate::aelhometta::{
        Ælhometta,
        Content,
        Uid
    },
    super::{
        DEFAULT_SEQUENCE_LIMIT,
        Commander,
        ParseErrorPrefixise,
        ParseHex
    }
};

impl Commander {
    pub fn backtrace(&self, æh: &Ælhometta, paramstr: &[&str]) -> Result<(), String> {
        let (snuid, limit) = if paramstr.len() > 0 {
            match paramstr[0].parse_hex::<Uid>() {
                Ok(uid) => {
                    if paramstr.len() > 1 {
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
                        (uid, DEFAULT_SEQUENCE_LIMIT)
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
            Some(_) => {
                let mut backlist = Vec::<(Uid, Content)>::new();
                let mut nuid = snuid;
                let mut nuids_set: HashSet<Uid> = HashSet::new();
                let mut n: usize = 0;

                // How it ends, besides "no previous node"?
                let mut end_loop = false;
                let mut end_limit = false;
                let mut end_empty = false;
                let mut end_multiple = 0usize;

                print!("{}", "Obtaining backward sequence".dark_blue());
                io::stdout().flush().unwrap_or(());
                loop {
                    print!("{}", ".".dark_blue());
                    io::stdout().flush().unwrap_or(());
                    let prev_nodes = æh.previous_nodes(&nuid);
                    match prev_nodes.len() {
                        1 => {
                            nuid = prev_nodes[0].0;
                            let content = prev_nodes[0].2;
                            backlist.push((nuid, content));
                            if ! nuids_set.insert(nuid) {
                                end_loop = true;
                                break;
                            };
                            n += 1;
                            if n >= limit {
                                end_limit = true;
                                break;
                            }
                        },
                        0 => {
                            end_empty = true;
                            break;
                        },
                        _ => {
                            end_multiple = prev_nodes.len();
                            break;
                        }
                    }
                }
                println!(" {}", "OK".dark_green().bold());

                if end_loop {
                    println!("{}", "LOOP".dark_yellow().bold());
                }
                if end_limit {
                    println!("{}", "LIMIT".dark_yellow().bold());
                }
                if end_empty {
                    println!("{}", "×".magenta());
                }
                if end_multiple > 0 {
                    println!("{}", format!("{} previous nodes", end_multiple).dark_yellow().bold());
                }

                backlist.reverse();

                for (nuid, content) in backlist {
                    println!("{}{}", format!("{:20}", Some(nuid).hexly()).magenta(), format!("{:?}", content).yellow());
                }

                Ok(())
            },

            None => Err(format!("Uid not found"))
        }
    }

}