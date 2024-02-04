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
    pub fn peer(&self, æh: &mut Ælhometta, paramstr: &[&str]) -> Result<(), String> {
        if paramstr.len() > 0 {
            let command = paramstr[0].to_lowercase();
            match command.as_str() {
                "share" => {
                    if paramstr.len() >= 2 {
                        let setting = paramstr[1].to_lowercase();
                        match setting.as_str() {
                            "size" => {
                                if paramstr.len() >= 3 {
                                    match paramstr[2].parse::<usize>() {
                                        Ok(size) => {
                                            æh.peer_share_size(size)?;
                                            println!("{}", "Size set".green());
                                            Ok(())
                                        },
                                        Err(err) => err.prefixised("size")
                                    }
                                } else {
                                    Err(String::from("Size not specified"))
                                }
                            },

                            "interval" => {
                                if paramstr.len() >= 3 {
                                    match paramstr[2].parse::<i64>() {
                                        Ok(interval) => {
                                            æh.peer_share_interval(interval)?;
                                            println!("{}", "Interval set".green());
                                            Ok(())
                                        },
                                        Err(err) => err.prefixised("interval")
                                    }
                                } else {
                                    Err(String::from("Interval not specified"))
                                }
                            },

                            "now" => {
                                match æh.peer_share_now() {
                                    Ok(_) => {
                                        println!("{}", "Shared".green());
                                        Ok(())
                                    },
                                    Err(err) => {
                                        Err(format!("Sharing error: {}", err.to_string()))
                                    }
                                }
                            },

                            _ => {
                                Err(String::from("Unknown setting"))
                            }
                        }
                    } else {
                        Err(String::from("Setting not specified"))
                    }
                }

                "update" => {
                    æh.peer_update();
                    println!("{}", "Updated".green());
                    Ok(())
                },

                "secret" => {
                    if paramstr.len() >= 2 {
                        æh.peer_secret(paramstr[1])?;
                        println!("{}", "Key set".green());
                        Ok(())
                    } else {
                        Err(String::from("Key not specified"))
                    }
                },

                "port" => {
                    if paramstr.len() >= 2 {
                        match paramstr[1].parse::<u16>() {
                            Ok(port) => {
                                æh.peer_port(port)?;
                                println!("{}", "Port set".green());
                                Ok(())
                            },
                            Err(err) => err.prefixised("port index")
                        }
                    } else {
                        Err(String::from("Port not specified"))
                    }
                },

                "torport" => {
                    if paramstr.len() >= 2 {
                        match paramstr[1].parse::<u16>() {
                            Ok(port) => {
                                æh.peer_torport(port)?;
                                println!("{}", "Tor proxy port set".green());
                                Ok(())
                            },
                            Err(err) => err.prefixised("port index")
                        }
                    } else {
                        Err(String::from("Port not specified"))
                    }
                },

                "torhost" => {
                    if paramstr.len() >= 2 {
                        æh.peer_torhost(paramstr[1])?;
                        println!("{}", "Tor proxy host set".green());
                        Ok(())
                    } else {
                        Err(String::from("Host not specified"))
                    }
                },

                "expose" => {
                    æh.peer_expose()?;
                    println!("{}", "Exposed".green());
                    Ok(())
                },

                "repose" => {
                    æh.peer_repose()?;
                    println!("{}", "Reposed".green());
                    Ok(())
                },

                "connect" => {
                    if paramstr.len() > 1 {
                        if paramstr.len() > 2 {
                            if paramstr.len() > 3 {
                                match paramstr[3].parse::<u16>() {
                                    Ok(port) => {
                                        æh.peer_connect(paramstr[1], paramstr[2], port)?;
                                        println!("{}", "Connection initiated".green());
                                        Ok(())
                                    },
                                    Err(err) => err.prefixised("port index")
                                }
                            } else {
                                Err(String::from("Port not specified"))
                            }
                        } else {
                            Err(String::from("Onion address not specified"))
                        }
                    } else {
                        Err(String::from("Public key not specified"))
                    }
                },

                "disconnect" => {
                    if paramstr.len() >= 2 {
                        æh.peer_disconnect(paramstr[1])?;
                        println!("{}", "Disconnection initiated".green());
                        Ok(())
                    } else {
                        Err(String::from("Public key not specified"))
                    }
                },

                "ether" => {
                    if paramstr.len() > 1 {
                        let mut found = false;
                        for peer in æh.other_peers() {
                            if peer.publickey() == paramstr[1] {
                                found = true;
                                if paramstr.len() > 2 {
                                    match paramstr[2].parse::<usize>() {
                                        Ok(start) => {
                                            let length: usize = if paramstr.len() > 3 {
                                                match paramstr[3].parse::<usize>() {
                                                    Ok(l) => {
                                                        if l > 0 {
                                                            l
                                                        } else {
                                                            return Err(String::from("Length must be greater than 0"));
                                                        }
                                                    },
                                                    Err(err) => {
                                                        return err.prefixised("range length");
                                                    }
                                                }
                                            } else {
                                                1
                                            };
                                            
                                            let ether_integers = peer.ether_integers();
                                            for chan in start..(start + length) {
                                                if chan < ether_integers.len() {
                                                    println!("{}{:8}{}", format!("{:>12}", chan).dark_blue(), " ", format!("{0}={0:X}h", ether_integers[chan]).blue());
                                                } else {
                                                    println!("{}{:8}{}", format!("{:>12}", chan).dark_red(), " ", "OUT OF BOUNDS".red());
                                                };                                
                                            }
                                        },
                                        Err(err) => return err.prefixised("range start")
                                    }
                                } else {
                                    return Err(String::from("Range start not specified"));
                                }
                                break;
                            }
                        }
                        if found {
                            Ok(())
                        } else {
                            Err(String::from("No peer with given public key among other peers"))
                        }
                    } else {
                        Err(String::from("Other peer not specified"))
                    }
                },

                "whitelist" => {
                    if paramstr.len() > 1 {
                        let subcommand = paramstr[1].to_lowercase();
                        match subcommand.as_str() {
                            "add" => {
                                if paramstr.len() > 2 {
                                    æh.peer_whitelist_add(paramstr[2])?;
                                    println!("{}", "Added".green());
                                    Ok(())
                                } else {
                                    Err(String::from("Public key not specified"))
                                }
                            },

                            "del" => {
                                if paramstr.len() > 2 {
                                    æh.peer_whitelist_del(paramstr[2])?;
                                    println!("{}", "Deleted".green());
                                    Ok(())
                                } else {
                                    Err(String::from("Public key not specified"))
                                }
                            },

                            "clear" => {
                                æh.peer_whitelist_clear();
                                println!("{}", "Cleared".green());
                                Ok(())
                            },

                            _ => {
                                Err(String::from("Unknown subsubcommand"))
                            }
                        }
                    } else {
                        Err(String::from("Setting not specified"))
                    }
                }
                
                _ => Err(String::from("Unknown subcommand"))
            }

        } else {
            println!("{}{}", format!("{:24}", "Share size").dark_blue(), format!("{}", æh.share_size()).blue());
            println!("{}{}", format!("{:24}", "Share interval (μs)").dark_green(), format!("{}", æh.share_interval()).green());
            println!("{}{}", format!("{:24}", "Last share").dark_green(), format!("{}.{:03} UTC", NaiveDateTime::from_timestamp_micros(æh.ut_last_share()).unwrap_or_default().format("%Y.%m.%d %a %H:%M:%S"), (æh.ut_last_share() / 1000) % 1000).green());
            println!("{}{}", format!("{:24}", "Secret key").dark_red(), format!("{}", & æh.secretkey()).red());
            println!("{}{}", format!("{:24}", "Port").dark_blue(), format!("{}", æh.port()).blue());
            println!("{}{}", format!("{:24}", "Tor proxy port").dark_blue(), format!("{}", æh.torproxy_port()).blue());
            println!("{}{}", format!("{:24}", "Tor proxy host").dark_magenta(), format!("{}", & æh.torproxy_host()).magenta());
            println!("{}{}", format!("{:24}", "Exposed").dark_yellow(), format!("{}", æh.exposed()).yellow());
            print!("{}", format!("{:24}", "Incoming absorbing").dark_blue());
            match æh.in_absorbing_num() {
                Some(num) => {
                    println!("{}", format!("{}", num).blue());
                },
                None => {
                    println!("{}", "Not exposed".dark_yellow().bold());
                }
            }
            println!("{}{}", format!("{:24}", "Incoming permitted").dark_blue(), format!("{}", æh.in_permitted_num()).blue());
            println!("{}{}", format!("{:24}", "Incoming attempted").dark_blue(), format!("{}", æh.in_attempted_num()).blue());
            println!("{:8}{}", " ", format!("Other peers ({})", æh.other_peers().len()).dark_grey());
            for (i, op) in æh.other_peers().iter().enumerate() {
                println!("{:4}{}", " ", format!("Peer {}", 1 + i).dark_grey());
                println!("{}{}", format!("{:24}", "Public key").dark_yellow(), format!("{}", & op.publickey()).yellow());
                println!("{}{}", format!("{:24}", "Onion").dark_magenta(), format!("{}", & op.onion()).magenta());
                println!("{}{}", format!("{:24}", "Port").dark_blue(), format!("{}", op.port()).blue());
                println!("{}{}", format!("{:24}", "Share size").dark_blue(), format!("{}", op.ether_integers().len()).blue());
                println!("{}{}", format!("{:24}", "Last update").dark_green(), format!("{}.{:03} UTC", NaiveDateTime::from_timestamp_micros(op.ut_last_update()).unwrap_or_default().format("%Y.%m.%d %a %H:%M:%S"), (op.ut_last_update() / 1000) % 1000).green());
            }
            Ok(())
        }
    }

}