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
        Commander,
        ParseErrorPrefixise
    }
};

impl Commander {
    pub fn glitch(&self, æh: &mut Ælhometta, paramstr: &[&str]) -> Result<(), String> {
        if paramstr.len() > 0 {
            let glitch_type = paramstr[0].to_lowercase();
            match glitch_type.as_str() {
                "back" => {
                    if paramstr.len() > 1 {
                        match paramstr[1].parse::<f64>() {
                            Ok(prob) => {
                                æh.set_glitch_background_prob(prob);
                                println!("{}", "Probability set".green());
                                Ok(())
                            },
                            Err(err) => err.prefixised("probability")
                        }
                    } else {
                        Err(String::from("Probability not specified"))
                    }
                },

                "repl" => {
                    if paramstr.len() > 1 {
                        match paramstr[1].parse::<f64>() {
                            Ok(prob) => {
                                æh.set_glitch_replicate_prob(prob);
                                println!("{}", "Probability set".green());
                                Ok(())
                            },
                            Err(err) => err.prefixised("probability")
                        }
                    } else {
                        Err(String::from("Probability not specified"))
                    }
                },

                "cons" => {
                    if paramstr.len() > 1 {
                        match paramstr[1].parse::<f64>() {
                            Ok(prob) => {
                                æh.set_glitch_construct_prob(prob);
                                println!("{}", "Probability set".green());
                                Ok(())
                            },
                            Err(err) => err.prefixised("probability")
                        }
                    } else {
                        Err(String::from("Probability not specified"))
                    }
                },

                _ => Err(String::from("Unknown type"))
            }            
        } else {
            println!("{:4}{}", " ", "Background".dark_grey());
            println!("{}{}", format!("{:16}", "Probability").dark_yellow(), format!("{:e}", æh.glitch_background_prob()).yellow());
            println!("{}{}", format!("{:16}", "Count").dark_blue(), format!("{}", æh.glitch_background_count()).blue());
            println!("{:4}{}", " ", "Replicate".dark_grey());
            println!("{}{}", format!("{:16}", "Probability").dark_yellow(), format!("{:e}", æh.glitch_replicate_prob()).yellow());
            println!("{}{}", format!("{:16}", "Count").dark_blue(), format!("{}", æh.glitch_replicate_count()).blue());
            println!("{:4}{}", " ", "Construct".dark_grey());
            println!("{}{}", format!("{:16}", "Probability").dark_yellow(), format!("{:e}", æh.glitch_construct_prob()).yellow());
            println!("{}{}", format!("{:16}", "Count").dark_blue(), format!("{}", æh.glitch_construct_count()).blue());
            Ok(())
        }
    }

}