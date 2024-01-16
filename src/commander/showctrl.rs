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
    crate::aelhometta::{
        Ælhometta,
        Hexly,
        Uid
    },
    super::{
        Commander,
        ParseErrorPrefixise,
        ParseHex
    }
};

impl Commander {
    pub fn showctrl(&self, æh: &Ælhometta, paramstr: &[&str]) -> Result<(), String> {
        if paramstr.len() >= 1 {
            match paramstr[0].parse_hex::<Uid>() {
                Ok(cuid) => {
                    match æh.controller(&cuid) {
                        Some(ctrl) => {
                            println!("{:8}{}", " ", "Self-chain & Data optuids".dark_grey());
                            println!("{}{}", format!("{:24}", "ChainStart").dark_magenta(), ctrl.chain_start_optuid().hexly().magenta());
                            println!("{}{}", format!("{:24}", "Exec").dark_magenta(), ctrl.exec_optuid().hexly().magenta());
                            print!("{}", format!("{:24}", "Data").dark_magenta());
                            let data_optuids = ctrl.data_optuids();
                            for i in 0..data_optuids.len() {
                                let mut ouid_hexly_stl = data_optuids[i].hexly().magenta();
                                if i == ctrl.i_data_optuid() {
                                    ouid_hexly_stl = ouid_hexly_stl.bold();
                                }
                                print!("{}{}", ouid_hexly_stl, ", ".dark_grey());
                            }
                            println!("");
                            println!("{}{}", format!("{:24}", "Data index").dark_blue(), format!("{}", ctrl.i_data_optuid()).blue());
                            println!("{:8}{}", " ", "New chain".dark_grey());
                            println!("{}{}", format!("{:24}", "Chain").dark_magenta(), ctrl.new_chain_optuid().hexly().magenta());
                            println!("{}{}", format!("{:24}", "Ctrl").dark_cyan(), format!("{}",if ctrl.new_controller_is_some() {"Some"} else {"None"}).blue());
                            println!("{:8}{}", " ", "Registers".dark_grey());
                            println!("{}{}", format!("{:24}", "Integer").dark_blue(), format!("{0}={0:X}h", ctrl.registers().integer()).blue());
                            println!("{:8}{}", " ", "Flags".dark_grey());
                            println!("{}{}", format!("{:24}", "Success").dark_blue(), format!("{}", ctrl.flags().success()).blue());
                            println!("{:8}{}", " ", "Node optuids".dark_grey());
                            print!("{}", format!("{:24}", "Optuids").dark_magenta());
                            let optuids = ctrl.optuids();
                            for i in 0..optuids.len() {
                                let mut ouid_hexly_stl = optuids[i].hexly().magenta();
                                if i == ctrl.i_optuid() {
                                    ouid_hexly_stl = ouid_hexly_stl.bold();
                                }
                                print!("{}{}", ouid_hexly_stl, ", ".dark_grey());
                            }
                            println!("");
                            println!("{}{}", format!("{:24}", "Optuid index").dark_blue(), format!("{}", ctrl.i_optuid()).blue());
                            println!("{:8}{}", " ", "Integers".dark_grey());
                            print!("{}", format!("{:24}", "Integers").dark_blue());
                            let integers = ctrl.integers();
                            for i in 0..integers.len() {
                                let mut int_stl = format!("{0}={0:X}h", integers[i]).blue();
                                if i == ctrl.i_integer() {
                                    int_stl = int_stl.bold();
                                }
                                print!("{}{}", int_stl, ", ".dark_grey());
                            }
                            println!("");
                            println!("{}{}", format!("{:24}", "Integer index").dark_blue(), format!("{}", ctrl.i_integer()).blue());
                            println!("{:8}{}", " ", "Optuid channels".dark_grey());
                            print!("{}", format!("{:24}", "Channels").dark_magenta());
                            let optuid_channels = ctrl.optuid_channels();
                            for i in 0..optuid_channels.len() {
                                let mut chan_stl = format!("{0}={0:X}h", optuid_channels[i]).magenta();
                                if i == ctrl.i_optuid_channel() {
                                    chan_stl = chan_stl.bold();
                                }
                                print!("{}{}", chan_stl, ", ".dark_grey());
                            }
                            println!("");
                            println!("{}{}", format!("{:24}", "Channel index").dark_blue(), format!("{}", ctrl.i_optuid_channel()).blue());
                            println!("{:8}{}", " ", "Integer channels".dark_grey());
                            print!("{}", format!("{:24}", "Channels").dark_blue());
                            let integer_channels = ctrl.integer_channels();
                            for i in 0..integer_channels.len() {
                                let mut chan_stl = format!("{0}={0:X}h", integer_channels[i]).blue();
                                if i == ctrl.i_integer_channel() {
                                    chan_stl = chan_stl.bold();
                                }
                                print!("{}{}", chan_stl, ", ".dark_grey());
                            }
                            println!("");
                            println!("{}{}", format!("{:24}", "Channel index").dark_blue(), format!("{}", ctrl.i_integer_channel()).blue());
                            println!("{:8}{}", " ", "Debug".dark_grey());
                            println!("{}{}", format!("{:24}", "Generation").dark_blue(), format!("{}", ctrl.generation()).blue());
                            println!("{}{}", format!("{:24}", "Ticks").dark_blue(), format!("{}", ctrl.ticks()).blue());
            
                            Ok(())
                        },
                        None => Err(format!("Uid not found"))
                    }
                },
                Err(err) => err.prefixised("ctrl uid")
            }
        } else {
            Err(String::from("Uid not specified"))
        }
    }

}