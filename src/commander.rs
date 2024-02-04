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

use serde::{
    Deserialize,
    Serialize
};

use std::{
    collections::VecDeque,
    fs,
    time::{
        SystemTime,
        UNIX_EPOCH
    }
};

mod ancestors;
mod backtrace;
mod changelim;
mod cleanse;
mod commandswitch;
mod ether;
mod glitch;
mod help;
mod history;
mod iomap;
mod peer;
mod prevnodes;
mod random;
mod run;
mod set;
mod settings;
mod shell;
mod showctrl;
mod shownode;
mod showseq;
mod showsizes;
mod statistics;
mod tick;

use crate::aelhometta::{
    Uid,
    Ælhometta
};

pub const DEFAULT_COMMANDER_FILENAME: &str = "commander.json";
pub const HISTORY_MAX_LEN: usize = 0x10000;

#[derive(Serialize, Deserialize)]
struct Settings {
    #[serde(default)] show_ticks: bool,
    #[serde(default)] show_abs_time: bool,
    #[serde(default = "def_sequence_def_limit")] sequence_def_limit: usize,
    #[serde(default = "def_show_freqs")] show_freqs: bool,
    #[serde(default = "def_freqs_interval")] freqs_interval: usize, // seconds
    #[serde(default = "def_freqs_window_margin")] freqs_window_margin: usize,
    #[serde(default = "def_freqs_comm_str_len")] freqs_comm_str_len: usize,
    #[serde(default = "def_freqs_cons_str_len")] freqs_cons_str_len: usize,
}

#[derive(Serialize, Deserialize)]
struct Record {
    command: String,
    timestamp: i64 // microseconds since Unix epoch
}

#[derive(Serialize, Deserialize)]
struct History {
    records: VecDeque<Record>
}

#[derive(Serialize, Deserialize)]
struct Selections {
    i_command: usize,
    i_construction: usize
}

#[derive(Serialize, Deserialize)]
pub struct Commander {
    settings: Settings,
    history: History,
    #[serde(default = "def_selections")] selections: Selections
}

pub trait ParseHex {
    fn parse_hex<N>(&self) -> Result<Uid, String>;
}

pub trait ParseErrorPrefixise {
    fn prefixised<T>(&self, prefix: &str) -> Result<T, String>;
}

fn def_show_freqs() -> bool {
    true
}

fn def_sequence_def_limit() -> usize {
    0x400
}

fn def_freqs_interval() -> usize {
    16
}

fn def_freqs_window_margin() -> usize {
    3
}

fn def_freqs_comm_str_len() -> usize {
    13
}

fn def_freqs_cons_str_len() -> usize {
    12
}

fn def_selections() -> Selections {
    Selections::new_default()
}

impl Settings {
    fn new_default() -> Self {
        Self {
            show_ticks: false,
            show_abs_time: false,
            sequence_def_limit: def_sequence_def_limit(),
            show_freqs: def_show_freqs(),
            freqs_interval: def_freqs_interval(),
            freqs_window_margin: def_freqs_window_margin(),
            freqs_comm_str_len: def_freqs_comm_str_len(),
            freqs_cons_str_len: def_freqs_cons_str_len()
        }
    }
}

impl Record {
    fn new_now(command: &str) -> Self {
        let command = String::from(command);
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |dur| dur.as_micros() as i64);
        Self {
            command,
            timestamp
        }
    }

    pub fn command(&self) -> String {
        self.command.clone()
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

}

impl History {
    fn new_default() -> Self {
        Self {
            records: VecDeque::new()
        }
    }

    pub fn add(&mut self, command: &str) {
        if self.records.len() >= HISTORY_MAX_LEN {
            self.records.pop_front();
        }
        self.records.push_back(Record::new_now(command));
    }

    pub fn past(&self, mut n: usize) -> Vec<&Record> {
        let l = self.records.len();
        n = n.min(l);
        self.records.range((l - n)..).collect::<Vec<&Record>>()
    }

    pub fn last(&self) -> Option<&Record> {
        self.records.back()
    }

}

impl Selections {
    fn new_default() -> Self {
        Self {
            i_command: 0,
            i_construction: 0
        }
    }

}

impl Commander {
    fn print_state(æh: &Ælhometta, full: bool) {
        print!("{} {} {} {} {} {} {} {}", 
            "Age".dark_blue(),
            format!("{}", æh.age()).blue(),
            ":".dark_grey(),
            "Nodes".dark_magenta(),
            format!("{}", æh.num_nodes()).magenta(),
            "|".dark_grey(),
            "Controllers".dark_cyan(),
            format!("{}", æh.num_controllers()).cyan()
        );
        if full {
            print!(" {} {} {} {}{} {}",
                "|".dark_grey(),
                "Limit".dark_red(),
                format!("{}=2^{}", æh.max_num_chains(), æh.max_num_chains_binlog()).red(),
                ": Memory ~ ".dark_grey(),
                format!("{}", æh.mem_usage() >> 20).dark_grey().bold(),
                "MiB".dark_grey()
            );
        }
    }

    pub fn new_default() -> Self {
        let settings = Settings::new_default();
        let history = History::new_default();
        let selections = Selections::new_default();
        Self {
            settings,
            history,
            selections
        }
    }

    pub fn save(&self, filepath: &str) -> Result<(), String> {
        let json = serde_json::to_vec_pretty(self).map_err(|err| format!("Cannot serialize Commander: {}", &err))?;
        fs::write(filepath, &json).map_err(|err| format!("Cannot write to '{}': {}", filepath, &err))?;
        Ok(())
    }

    pub fn save_default(&self) -> Result<(), String> {
        self.save(DEFAULT_COMMANDER_FILENAME)
    }

    pub fn load(filepath: &str) -> Result<Self, String> {
        let json = fs::read(filepath).map_err(|err| format!("Cannot read from '{}': {}", filepath, &err))?;
        let commander: Self = serde_json::from_slice(&json).map_err(|err| format!("Cannot deserialize Commander: {}", &err))?;
        Ok(commander)
    }

    pub fn load_default() -> Result<Self, String> {
        Self::load(DEFAULT_COMMANDER_FILENAME)
    }

}

impl<S: ToString> ParseHex for S {
    fn parse_hex<N>(&self) -> Result<u32, String> {
        let chars = self.to_string().chars().collect::<Vec<char>>();
        let mut n = 0;
        for c in chars {
            n = (n << 4) | match c.to_digit(0x10) {
                Some(d) => d as u32,
                None => {
                    return Err(format!("'{}' is not a hexadecimal char", c))
                }
            }            
        }
        Ok(n)
    }
}


impl<ES: ToString> ParseErrorPrefixise for ES {
    fn prefixised<T>(&self, prefix: &str) -> Result<T, String> {
        Err(format!("Parse error: {}: {}", prefix, self.to_string()))
    }
}
