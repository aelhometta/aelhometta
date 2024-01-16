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

use {
    crate::aelhometta::Ælhometta,
    super::{
        Commander,
        ParseErrorPrefixise
    }
};

impl Commander {
    pub fn iomap(&self, æh: &mut Ælhometta, paramstr: &[&str]) -> Result<(), String> {
        if paramstr.len() > 0 {
            let command = paramstr[0].to_lowercase();
            match command.as_str() {
                "out" => {
                    if paramstr.len() > 1 {
                        let subcommand = paramstr[1].to_lowercase();
                        match subcommand.as_str() {
                            "add" => {
                                if paramstr.len() > 2 {
                                    match paramstr[2].parse::<usize>() {
                                        Ok(start) => {
                                            if paramstr.len() > 3 {
                                                match paramstr[3].parse::<usize>() {
                                                    Ok(length) => {
                                                        if paramstr.len() > 4 {
                                                            match paramstr[4].parse::<i64>() {
                                                                Ok(interval) => {
                                                                    if paramstr.len() > 5 {
                                                                        æh.iomap_out_add(start, length, interval, paramstr[5])?;
                                                                        println!("{}", "Mapping added".green());
                                                                        Ok(())
                                                                    } else {
                                                                        Err(String::from("Filepath not specified"))
                                                                    }
                                                                },
                                                                Err(err) => err.prefixised("mapping interval")
                                                            }
                                                        } else {
                                                            Err(String::from("Mapping interval not specified"))
                                                        }
                                                    },
                                                    Err(err) => err.prefixised("range length")
                                                }
                                            } else {
                                                Err(String::from("Range length not specified"))
                                            }
                                        },
                                        Err(err) => err.prefixised("range start")
                                    }
                                } else {
                                    Err(String::from("Range start not specified"))
                                }
                            },

                            "del" => {
                                if paramstr.len() > 2 {
                                    match paramstr[2].parse::<usize>() {
                                        Ok(index) => {
                                            æh.iomap_out_del(index)?;
                                            println!("{}", "Mapping removed".green());
                                            Ok(())
                                        },
                                        Err(err) => err.prefixised("index")
                                    }
                                } else {
                                    Err(String::from("Index not specified"))
                                }
                            },

                            "list" => {
                                for (i, om) in æh.output_mappings().iter().enumerate() {
                                    println!("{:4}{}", " ", format!("Output mapping {}", i).dark_grey());
                                    println!("{}{}", format!("{:24}", "Start").dark_blue(), format!("{}", om.start()).blue());
                                    println!("{}{}", format!("{:24}", "Length").dark_blue(), format!("{}", om.length()).blue());
                                    println!("{}{}", format!("{:24}", "Interval (μs)").dark_green(), format!("{}", om.interval()).green());
                                    println!("{}{}", format!("{:24}", "Filepath").dark_magenta(), format!("{}", & om.filepath()).magenta());
                                    println!("{}{}", format!("{:24}", "Last update").dark_green(), format!("{}.{:03} UTC", NaiveDateTime::from_timestamp_micros(om.ut_last_update()).unwrap_or_default().format("%Y.%m.%d %a %H:%M:%S"), (om.ut_last_update() / 1000) % 1000).green());
                                }
                                Ok(())
                            },

                            _ => Err(String::from("Unknown subsubcommand"))
                        }
                    } else {
                        Err(String::from("Subsubcommand not specified"))
                    }
                },

                "in" => {
                    if paramstr.len() > 1 {
                        let subcommand = paramstr[1].to_lowercase();
                        match subcommand.as_str() {
                            "add" => {
                                if paramstr.len() > 2 {
                                    match paramstr[2].parse::<usize>() {
                                        Ok(start) => {
                                            if paramstr.len() > 3 {
                                                match paramstr[3].parse::<usize>() {
                                                    Ok(length) => {
                                                        if paramstr.len() > 4 {
                                                            match paramstr[4].parse::<i64>() {
                                                                Ok(interval) => {
                                                                    if paramstr.len() > 5 {
                                                                        æh.iomap_in_add(start, length, interval, paramstr[5])?;
                                                                        println!("{}", "Mapping added".green());
                                                                        Ok(())
                                                                    } else {
                                                                        Err(String::from("Filepath not specified"))
                                                                    }
                                                                },
                                                                Err(err) => err.prefixised("mapping interval")
                                                            }
                                                        } else {
                                                            Err(String::from("Mapping interval not specified"))
                                                        }
                                                    },
                                                    Err(err) => err.prefixised("range length")
                                                }
                                            } else {
                                                Err(String::from("Range length not specified"))
                                            }
                                        },
                                        Err(err) => err.prefixised("range start")
                                    }
                                } else {
                                    Err(String::from("Range start not specified"))
                                }
                            },

                            "del" => {
                                if paramstr.len() > 2 {
                                    match paramstr[2].parse::<usize>() {
                                        Ok(index) => {
                                            æh.iomap_in_del(index)?;
                                            println!("{}", "Mapping removed".green());
                                            Ok(())
                                        },
                                        Err(err) => err.prefixised("index")
                                    }
                                } else {
                                    Err(String::from("Index not specified"))
                                }
                            },

                            "list" => {
                                for (i, im) in æh.input_mappings().iter().enumerate() {
                                    println!("{:4}{}", " ", format!("Input mapping {}", i).dark_grey());
                                    println!("{}{}", format!("{:24}", "Start").dark_blue(), format!("{}", im.start()).blue());
                                    println!("{}{}", format!("{:24}", "Length").dark_blue(), format!("{}", im.length()).blue());
                                    println!("{}{}", format!("{:24}", "Interval (μs)").dark_green(), format!("{}", im.interval()).green());
                                    println!("{}{}", format!("{:24}", "Filepath").dark_magenta(), format!("{}", & im.filepath()).magenta());
                                    println!("{}{}", format!("{:24}", "Last update").dark_green(), format!("{}.{:03} UTC", NaiveDateTime::from_timestamp_micros(im.ut_last_update()).unwrap_or_default().format("%Y.%m.%d %a %H:%M:%S"), (im.ut_last_update() / 1000) % 1000).green());
                                }
                                Ok(())
                            },

                            _ => Err(String::from("Unknown subsubcommand"))
                        }
                    } else {
                        Err(String::from("Subsubcommand not specified"))
                    }
                },

                "update" => {
                    æh.iomap_update();
                    println!("{}", "Updated".green());
                    Ok(())
                },

                _ => Err(String::from("Unknown subcommand"))
            }
        } else {
            Err(String::from("Subcommand not specified"))
        }
    }

}
