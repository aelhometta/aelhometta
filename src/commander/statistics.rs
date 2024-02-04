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
    collections::BTreeMap,
    io::{
        self,
        Write
    }
};

use {
    crate::{
        aelhometta::{
            Ælhometta,
            CONTENTS
        },
        serbin::ToBits
    },
    super::Commander
};

impl Commander {
    pub fn statistics(&self, æh: &Ælhometta, paramstr: &[&str]) -> Result<(), String> {
        if paramstr.len() >= 1 {
            let topic = paramstr[0].to_lowercase();
            match topic.as_str() {
                "cgen" => {
                    let cg_stats = æh.generation_statistics();
                    println!("{}{}", format!("{:12}", "Minimum").dark_blue(), format!("{:>12}", cg_stats.minimum).blue());
                    println!("{}{}", format!("{:12}", "Average").dark_blue(), format!("{:>12}", cg_stats.average).blue());
                    println!("{}{}", format!("{:12}", "Maximum").dark_blue(), format!("{:>12}", cg_stats.maximum).blue());
                    Ok(())
                },

                "chan" => {
                    let ch_stats = æh.channels_statistics();
                    println!("{} {} {}{}{}",
                        "Optuid:".dark_magenta(),
                        "non-none".dark_grey(),
                        format!("{}", ch_stats.optuids_some).magenta().bold(),
                        "/".dark_grey(),
                        format!("{}", æh.ether_optuids().len()).magenta()
                    );
                    println!("{} {} {}{}{}",
                        "Integer:".dark_blue(),
                        "non-zero".dark_grey(),
                        format!("{}", ch_stats.integers_nonzero).blue().bold(),
                        "/".dark_grey(),
                        format!("{}", æh.ether_integers().len()).blue()
                    );
                    Ok(())
                },
    
                "cont" => {
                    print!("{}", "Obtaining content statistics... ".dark_blue());
                    io::stdout().flush().unwrap_or(());
                    let cont_stats = æh.content_statistics();
                    println!("{}", "OK".dark_green().bold());
    
                    for content in CONTENTS {
                        let count = * cont_stats.get(&content).unwrap_or(&0);
                        println!("{}{}{}{}",
                            format!("{:<4}", format!("{:02X}", content.to_bits())).blue(),
                            format!("{:<40}", format!("{:?}", content)).yellow(),
                            format!("{:>12}", count).dark_yellow(),
                            format!("{:>11.3} %", ((count) as f64) * 100.0 / (æh.num_nodes().max(1) as f64)).dark_blue()
                        );
                    }
                    Ok(())
                },
    
                "tick" => {
                    println!("{}{}",
                        format!("{:32}", "Spaces").dark_yellow(),
                        format!("{:>16}", æh.spaces_count()).blue()
                    );

                    println!("{:4}{}", " ", "Branches".dark_grey());
                    println!("{}{}",
                        format!("{:32}", "Branches (main)").dark_magenta(),
                        format!("{:>16}", æh.branches_main_count()).blue()
                    );
                    println!("{}{}",
                        format!("{:32}", "Branches (alt)").dark_magenta(),
                        format!("{:>16}", æh.branches_alt_count()).blue()
                    );

                    println!("{:4}{}", " ", "Commands".dark_grey());
                    let mut comm_str_count: BTreeMap<String, u128> = BTreeMap::new();
                    for (command, count) in æh.commands_count() {
                        comm_str_count.insert(format!("{:?}", *command), *count);
                    }
                    for (comm_str, count) in &comm_str_count {
                        println!("{}{}",
                            format!("{:<32}", comm_str).yellow(),
                            format!("{:>16}", *count).blue()
                        );
                    }

                    println!("{:4}{}", " ", "Constructions".dark_grey());
                    let mut cons_str_count: BTreeMap<String, u128> = BTreeMap::new();
                    for (construction, count) in æh.constructions_count() {
                        cons_str_count.insert(format!("{:?}", *construction), *count);
                    }
                    for (cons_str, count) in &cons_str_count {
                        println!("{}{}",
                            format!("{:<32}", cons_str).magenta(),
                            format!("{:>16}", *count).blue()
                        );
                    }

                    Ok(())
                },
    
                _ => {
                    Err(format!("Unknown topic '{}'", topic))
                }
            }
        } else {
            Err(String::from("Topic not specified"))
        }
    }

}