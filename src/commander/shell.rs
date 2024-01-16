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

use crossterm::{
    cursor,
    ExecutableCommand,
    style::Stylize
};

use std::io::{
    self,
    Write
};

use {
    crate::aelhometta::Ælhometta,
    super::Commander
};

impl Commander {
    pub fn shell(&mut self, æh: &mut Ælhometta) -> Result<bool, String> {
        let _ = io::stdout().execute(cursor::Show);

        let mut hint_shown: bool = false;

        let do_save = loop {
            println!("");

            // State
            Self::print_state(æh, true);
            println!("");

            // Hint
            if !hint_shown {
                println!("{}", "\"?\" — see the list of available commands".dark_grey());
                hint_shown = true;
            }

            print!("@ ");
            io::stdout().flush().unwrap_or(());

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(..) => {
                    input.pop(); // remove trailing newline
                    
                    // In-place substitution in case of "repeat last command"
                    if input.as_str() == "=" {
                        match self.history.last() {
                            Some(rec) => {
                                input = rec.command();
                                println!("@ {}", &input);
                            },
                            None => {
                                input.clear();
                                println!("×");
                            }
                        }
                    }

                    let tokens: Vec<&str> = input.split_whitespace().collect();
                    if tokens.len() > 0 {
                        let command = tokens[0];
                        match command {
                            "q" | "quit" | "exit" | "end" | "bye" => {
                                break true;
                            },

                            "qq" | "quitquit" | "exitexit" | "endend" | "byebye" => {
                                break false;
                            },

                            "help" | "?" => {
                                let comm = if tokens.len() > 1 {
                                    tokens[1]
                                } else {
                                    ""
                                };
                                match self.help(comm) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error showing help: {}", &err).red().bold());
                                    }
                                }
                            },

                            "anc" | "ancestor" => {
                                match self.ancestors(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error introducing ancestor: {}", &err).red().bold());
                                    }
                                }
                            },

                            "r" | "run" => {
                                match self.run(æh, None) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error running: {}", &err).red().bold());
                                    }
                                }
                            },
                                                        
                            "t" | "tick" => {
                                match self.tick(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error running ticks: {}", &err).red().bold());
                                    }
                                }
                            },

                            "glitch" => {
                                match self.glitch(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error changing glitch probability: {}", &err).red().bold());
                                    }
                                }
                            },

                            "sn" | "shownode" => {
                                match self.shownode(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error showing node: {}", &err).red().bold());
                                    }
                                }
                            },

                            "sct" | "showctrl" => {
                                match self.showctrl(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error showing controller: {}", &err).red().bold());
                                    }
                                }
                            },

                            "ss" | "showseq" => {
                                match self.showseq(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error showing forward sequence: {}", &err).red().bold());
                                    }
                                }
                            },

                            "prev" | "prevnodes" => {
                                match self.prevnodes(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error showing previous nodes: {}", &err).red().bold());
                                    }
                                }
                            },

                            "back" | "backtrace" => {
                                match self.backtrace(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error showing backward sequence: {}", &err).red().bold());
                                    }
                                }
                            },

                            "eth" | "ether" => {
                                match self.ether(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error showing ether: {}", &err).red().bold());
                                    }
                                }
                            },

                            "rand" | "random" => {
                                match self.random(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error obtaining random identifier: {}", &err).red().bold());
                                    }
                                }
                            },

                            "stat" | "statistics" => {
                                match self.statistics(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error showing statistics: {}", &err).red().bold());
                                    }
                                }
                            },

                            "cleanse" => {
                                match self.cleanse(æh) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error cleansing: {}", &err).red().bold());
                                    }
                                }
                            },

                            "introspection" => {
                                match self.introspection(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error configuring introspection: {}", &err).red().bold());
                                    }
                                }
                            },

                            "changelim" => {
                                match self.changelim(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error changing limits: {}", &err).red().bold());
                                    }
                                }
                            },

                            "p" | "peer" => {
                                match self.peer(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error configuring peer: {}", &err).red().bold());
                                    }
                                }
                            },

                            "iomap" => {
                                match self.iomap(æh, & tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error configuring input/output mappings: {}", &err).red().bold());
                                    }
                                }
                            },

                            "showsizes" => {
                                match self.showsizes(æh) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error showing sizes: {}", &err).red().bold());
                                    }
                                }
                            },

                            "sets" | "settings" => {
                                let setting = if tokens.len() > 1 {
                                    tokens[1]
                                } else {
                                    ""
                                };
                                match self.settings(setting) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error showing setting: {}", &err).red().bold());
                                    }
                                }
                            },

                            "set" => {
                                match self.set(& tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error setting: {}", &err).red().bold());
                                    }
                                }
                            },

                            "hist" | "history" => {
                                match self.history(& tokens[1..]) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("{}", format!("Error showing history: {}", &err).red().bold());
                                    }
                                }
                            },

                            _ => {
                                println!("{}", "Unknown command".red().bold());
                            }
                        }

                    } else {
                        println!("{}", "No command, nothing to do".dark_yellow().bold());
                    }

                    self.history.add(&input);
                },
                Err(err) => {
                    println!("{}", format!("Error reading input: {}", err.to_string()).red().bold());
                }
            }
        };
        
        Ok(do_save)
    }

}