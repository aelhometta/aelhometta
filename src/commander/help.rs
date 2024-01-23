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

use emyzelium::{
    DEF_PUBSUB_PORT,
    DEF_TOR_PROXY_HOST,
    DEF_TOR_PROXY_PORT
};

use super::Commander;

fn print_short_commands_helps(commhelps: & Vec<(&str, &str)>) {
    for (comm, help) in commhelps {
        println!("{}{}", format!("{:32}", comm).dark_grey().bold(), help.dark_grey());
    }    
}

impl Commander {
    pub fn help(&self, command: &str) -> Result<(), String> {
        match command {
            "" => {
                print_short_commands_helps(& vec![
                    ("q | quit | exit | end | bye",
                        "Time to say goodbye, save state"),
                    ("qq | quitquit | ... | byebye",
                        "Time to say goodbye, do not save state"),
                    ("help | ? [command]",
                        "Show (this) list of available commands or help on given one"),
                    ("=",
                        "Repeat last command"),
                    ("anc | ancestor",
                        "Introduce ancestor of given kin with given parameters"),
                    ("r | run",
                        "Run until keypress"),
                    ("t | tick",
                        "Run given number of ticks of given controller"),
                    ("glitch",
                        "Probabilities and counts of random content changes, show or set"),
                    ("sn | shownode",
                        "Show given node"),
                    ("sct | showctrl",
                        "Show given controller"),
                    ("ss | showseq",
                        "Show forward sequence of nodes"),
                    ("prev | prevnodes",
                        "Show nodes that have given one as next"),
                    ("back | backtrace",
                        "Show backward sequence of nodes"),
                    ("eth | ether",
                        "Show given range of channels of given type"),
                    ("rand | random",
                        "Show identifier of random entity"),
                    ("stat | statistics",
                        "Show statistics on given topic"),
                    ("cleanse",
                        "Remove all nodes and controllers"),
                    ("introspection",
                        "Show or set switch allowing more access to an exec pointer"),
                    ("changelim",
                        "Change maximum number of chains"),
                    ("p | peer",
                        "Configure peer"),
                    ("iomap",
                        "Configure input/output mappings of integer channels"),
                    ("showsizes",
                        "Show constant sizes of some arrays"),
                    ("sets | settings",
                        "Show value(s) of commander setting(s)"),
                    ("set",
                        "Set commander setting to value"),
                    ("hist | history",
                        "Show history of commands")
                ]);
            },

            "q" | "quit" | "exit" | "end" | "bye" => {
                println!("{}{}", format!("{:32}", "quit | exit | end | bye").dark_grey().bold(), "Save current Ælhometta's and Commander's states and quit".dark_grey());
            },

            "qq" | "quitquit" | "exitexit" | "endend" | "byebye" => {
                println!("{}{}", format!("{:32}", "quitquit | ... | byebye").dark_grey().bold(), "Quit, do not save current Ælhometta's and Commander's states".dark_grey());
            },

            "help" | "?" => {
                println!("{}{}{}", format!("{:32}", "help [command]").dark_grey().bold(), "Show list of available commands or help on ".dark_grey(), "command".dark_grey().italic());
            },

            "=" => {
                println!("{}{}", format!("{:32}", "=").dark_grey().bold(), "Repeat last command, if any".dark_grey());
            }

            "anc" | "ancestor" => {
                println!("{}{}{}", format!("{:32}", "ancestor <kin> [<parameters>]").dark_grey().bold(), "Introduce ancestor of ".dark_grey(), "kin".dark_grey().italic());
                println!("{:32}{}{}", "", "kin".dark_grey().italic(), " : A, B, C, ... (case-insensitive)".dark_grey());
                println!("{:32}{}{}", "", "parameters".dark_grey().italic(), " : depend on kin, e.g. integer(s) in decimal".dark_grey());
                println!("{:40}{}{}", "", "A ".dark_grey(), "sterile_power skip_power spacity".dark_grey().italic());
                println!("{:40}{}{}", "", "B ".dark_grey(), "spacity".dark_grey().italic());
            },

            "r" | "run" => {
                println!("{}{}", format!("{:32}", "run").dark_grey().bold(), "Run ticks until keypress".dark_grey());
            },

            "t" | "tick" => {
                println!("{}{}{}{}{}{}", format!("{:32}", "tick [uid [count]] ").dark_grey().bold(), "Run ".dark_grey(), "count".dark_grey().italic(), " ticks of ".dark_grey(), "uid".dark_grey().italic(), "-specified controller".dark_grey());
                println!("{:32}{}{}", "", "uid".dark_grey().italic(), " : unsigned 64-bit integer in hexadecimal (case-insensitive), default is random".dark_grey());
                println!("{:32}{}{}", "", "count".dark_grey().italic(), " : positive integer in decimal, default is 1".dark_grey());
            },

            "glitch" => {
                println!("{}{}{}{}{}", format!("{:32}", "glitch <type> <prob>").dark_grey().bold(), "Set probability of ".dark_grey(), "type".dark_grey().italic(), " glitch to ".dark_grey(), "prob".dark_grey().italic());
                println!("{:32}{}{}", "", "type".dark_grey().italic(), " : one of the following (case-insensitive):".dark_grey());
                println!("{:38}{}{}", "", "back".dark_grey().italic(), " — 'background', each tick, arbitrary node".dark_grey());
                println!("{:38}{}{}", "", "repl".dark_grey().italic(), " — at replication, each copied node".dark_grey());
                println!("{:38}{}{}", "", "cons".dark_grey().italic(), " — at construction, each read node".dark_grey());
                println!("{:32}{}{}", "", "prob".dark_grey().italic(), " : floating point value in [0; 1]".dark_grey());
                println!("{}{}", format!("{:32}", "glitch").dark_grey().bold(), "Show probabilities and counts of aforementioned types of glitches".dark_grey());
            },

            "sn" | "shownode" => {
                println!("{}{}{}", format!("{:32}", "shownode <uid>").dark_grey().bold(), "Show node with ".dark_grey(), "uid".dark_grey().italic());
                println!("{:32}{}{}", "", "uid".dark_grey().italic(), " : unsigned 64-bit integer in hexadecimal (case-insensitive)".dark_grey());
            },

            "sct" | "showctrl" => {
                println!("{}{}{}", format!("{:32}", "showctrl <uid>").dark_grey().bold(), "Show controller with ".dark_grey(), "uid".dark_grey().italic());
                println!("{:32}{}{}", "", "uid".dark_grey().italic(), " : unsigned 64-bit integer in hexadecimal (case-insensitive)".dark_grey());
            },

            "ss" | "showseq" => {
                println!("{}{}{}{}{}", format!("{:32}", "showseq <uid> [limit]").dark_grey().bold(), "Show forward sequence of nodes starting from ".dark_grey(), "uid".dark_grey().italic(), ", maximum length is ".dark_grey(), "limit".dark_grey().italic());
                println!("{:32}{}{}", "", "uid".dark_grey().italic(), " : unsigned 64-bit integer in hexadecimal (case-insensitive)".dark_grey());
                println!("{:32}{}{}", "", "limit".dark_grey().italic(), " : positive integer in decimal, default is 1024".dark_grey());
            },

            "prev" | "prevnodes" => {
                println!("{}{}{}{}", format!("{:32}", "prevnodes <uid>").dark_grey().bold(), "Show nodes that have ".dark_grey(), "uid".dark_grey().italic(), " one as their next, main or alternative".dark_grey());
                println!("{:32}{}{}", "", "uid".dark_grey().italic(), " : unsigned 64-bit integer in hexadecimal (case-insensitive)".dark_grey());
            },

            "back" | "backtrace" => {
                println!("{}{}{}{}{}", format!("{:32}", "backtrace <uid> [limit]").dark_grey().bold(), "Show backward sequence of nodes starting from ".dark_grey(), "uid".dark_grey().italic(), ", maximum length is ".dark_grey(), "limit".dark_grey().italic());
                println!("{:32}{}{}", "", "uid".dark_grey().italic(), " : unsigned 64-bit integer in hexadecimal (case-insensitive)".dark_grey());
                println!("{:32}{}{}", "", "limit".dark_grey().italic(), " : positive integer in decimal, default is 1024".dark_grey());
            },

            "eth" | "ether" => {
                println!("{}{}{}{}{}{}{}{}", format!("{:32}", "ether <type> <start> [length]").dark_grey().bold(), "Show content of ".dark_grey(), "length".dark_grey().italic(), " channels of ".dark_grey(), "type".dark_grey().italic(), ", starting from ".dark_grey(), "start".dark_grey().italic(), "-th".dark_grey());
                println!("{:32}{}{}", "", "type".dark_grey().italic(), " : ouid (optuid) OR int (integer)".dark_grey());
                println!("{:32}{}{}", "", "start".dark_grey().italic(), " : unsigned integer in decimal".dark_grey());
                println!("{:32}{}{}", "", "length".dark_grey().italic(), " : positive integer in decimal, default is 1".dark_grey());
            },

            "rand" | "random" => {
                println!("{}{}{}", format!("{:32}", "random <entity_type>").dark_grey().bold(), "Show identifier of random entity of ".dark_grey(), "entity_type".dark_grey().italic());
                println!("{:32}{}{}", "", "entity_type".dark_grey().italic(), " : one of the following (case-insensitive):".dark_grey());
                println!("{:40}{}{}", "", "ctrl".dark_grey().italic(), " — controller".dark_grey());
                println!("{:40}{}{}", "", "node".dark_grey().italic(), " — node".dark_grey());
            },

            "stat" | "statistics" => {
                println!("{}{}{}", format!("{:32}", "statistics <topic>").dark_grey().bold(), "Show statistics related to ".dark_grey(), "topic".dark_grey().italic());
                println!("{:32}{}{}", "", "topic".dark_grey().italic(), " : one of the following (case-insensitive):".dark_grey());
                println!("{:39}{}{}", "", "cgen".dark_grey().italic(), " — generation of controllers".dark_grey());
                println!("{:39}{}{}", "", "chan".dark_grey().italic(), " — channels usage (optuid, integer)".dark_grey());
                println!("{:39}{}{}", "", "cont".dark_grey().italic(), " — content of nodes".dark_grey());
                println!("{:39}{}{}", "", "tick".dark_grey().italic(), " — execution count (spaces, branches, commands), construction instructions count".dark_grey());
            },

            "cleanse" => {
                println!("{}{}", format!("{:32}", "cleanse").dark_grey().bold(), "Reset by removing all nodes and controllers".dark_grey());
            },

            "introspection" => {
                println!("{}{}", format!("{:32}", "introspection [false|true]").dark_grey().bold(), "Show or set switch that unlocks GetExecFromOptuid and SetOptuidFromExec commands".dark_grey());
            },

            "changelim" => {
                println!("{}{}{}", format!("{:32}", "changelim <max_num_log2>").dark_grey().bold(), "Change maximum number of nodes and controllers to 2^".dark_grey(), "max_num_log2".dark_grey().italic());
                println!("{:32}{}{}", "", "max_num_log2".dark_grey().italic(), " : unsigned integer in decimal".dark_grey());
            },

            "p" | "peer" => {
                println!("{}{}{}{}", format!("{:40}", "peer <subcommand> [<parameters>]").dark_grey().bold(), "Execute ".dark_grey(), "subcommand".dark_grey().italic(), " related to peer config".dark_grey());
                println!("{}", "Available subcommands (E-/E+ requires to execute before/after exposition):".dark_grey());
                println!("{:5}{}{}", "", format!("{:35}", "info").dark_grey().bold(), "Show peer config and state".dark_grey());
                println!("{:5}{}{}{}{}", "", format!("{:35}", "share size <size>").dark_grey().bold(), "Set number of shared integer channels to ".dark_grey(), "size".dark_grey().italic(), ", starting from 0th".dark_grey());
                println!("{:40}{}{}", "", "size".dark_grey().italic(), " : unsigned integer in decimal, default is 0 - empty (not absent) share".dark_grey());
                println!("{:5}{}{}{}", "", format!("{:35}", "share interval <interval>").dark_grey().bold(), "Set interval between updating shared data to ".dark_grey(), "interval".dark_grey().italic());
                println!("{:40}{}{}", "", "interval".dark_grey().italic(), " : microseconds, unsigned integer in decimal, default is 0 - never share".dark_grey());
                println!("{:5}{}{}", "", format!("{:35}", "share now").dark_grey().bold(), "Share data now, do not wait for interval expiration. E+".dark_grey());
                println!("{:5}{}{}", "", format!("{:35}", "update").dark_grey().bold(), "Update peer state w.r.t. other peers and interval. E+".dark_grey());
                println!("{:5}{}{}{}{}", "", format!("{:35}", "secret <secretkey>").dark_grey().bold(), "Set secret key of this peer to ".dark_grey(), "secretkey".dark_grey().italic(), ". E-".dark_grey());
                println!("{:40}{}{}", "", "secretkey".dark_grey().italic(), " : 40-character CURVE secret key in Z85 encoding".dark_grey());
                println!("{:5}{}{}{}{}", "", format!("{:35}", "port <number>").dark_grey().bold(), "Set port of this peer to ".dark_grey(), "port".dark_grey().italic(), ". E-".dark_grey());
                println!("{:40}{}{}", "", "port".dark_grey().italic(), format!(" : unsigned integer from 0–65535 in decimal, default is {}", DEF_PUBSUB_PORT).dark_grey());
                println!("{:5}{}{}{}{}", "", format!("{:35}", "torport <number>").dark_grey().bold(), "Set Tor proxy port of this peer to ".dark_grey(), "port".dark_grey().italic(), ". E-".dark_grey());
                println!("{:40}{}{}", "", "port".dark_grey().italic(), format!(" : unsigned integer from 0–65535 in decimal, default is {}", DEF_TOR_PROXY_PORT).dark_grey());
                println!("{:5}{}{}{}{}", "", format!("{:35}", "torhost <host>").dark_grey().bold(), "Set Tor proxy host of this peer to ".dark_grey(), "host".dark_grey().italic(), ". E-".dark_grey());
                println!("{:40}{}{}", "", "host".dark_grey().italic(), format!(" : IP address, default is '{}'", DEF_TOR_PROXY_HOST).dark_grey());
                println!("{:5}{}{}", "", format!("{:35}", "expose").dark_grey().bold(), "Start network activity of this peer. E-".dark_grey());
                println!("{:5}{}{}", "", format!("{:35}", "repose").dark_grey().bold(), "Cease network activity of this peer. E+".dark_grey());
                println!("{:5}{}{}{}{}{}{}{}{}", "", format!("{:35}", "connect <publickey> <onion> <port>").dark_grey().bold(), "Connect to other peer with ".dark_grey(), "publickey".dark_grey().italic(), " at ".dark_grey(), "onion".dark_grey().italic(), ".onion:".dark_grey(), "port".dark_grey().italic(), ". E+".dark_grey());
                println!("{:40}{}{}", "", "publickey".dark_grey().italic(), " : 40-character CURVE public key in Z85 encoding".dark_grey());
                println!("{:40}{}{}", "", "onion".dark_grey().italic(), " : 56-character onion address (without '.onion')".dark_grey());
                println!("{:40}{}{}", "", "port".dark_grey().italic(), " : unsigned integer from 0–65535 in decimal".dark_grey());
                println!("{:5}{}{}{}{}", "", format!("{:35}", "disconnect <publickey>").dark_grey().bold(), "Disconnect from other peer with ".dark_grey(), "publickey".dark_grey().italic(), ". E+ and after connection initiated".dark_grey());
                println!("{:40}{}{}", "", "publickey".dark_grey().italic(), " : 40-character CURVE public key in Z85 encoding".dark_grey());
                println!("{:5}{}{}{}{}{}{}{}{}", "", format!("{:35}", "ether <publickey> <start> [length]").dark_grey().bold(), "Show Integer ether of other peer with ".dark_grey(), "publickey".dark_grey().italic(), ": ".dark_grey(), "length".dark_grey().italic(), " channels, starting from ".dark_grey(), "start".dark_grey().italic(), "-th".dark_grey());
                println!("{:40}{}{}", "", "publickey".dark_grey().italic(), " : 40-character CURVE public key in Z85 encoding".dark_grey());
                println!("{:40}{}{}", "", "start".dark_grey().italic(), " : unsigned integer in decimal".dark_grey());
                println!("{:40}{}{}", "", "length".dark_grey().italic(), " : positive integer in decimal, default is 1".dark_grey());
                println!("{:5}{}{}{}{}", "", format!("{:35}", "whitelist <add|del> <publickey>").dark_grey().bold(), "Add or delete other peer with ".dark_grey(), "publickey".dark_grey().italic(), " to whitelist".dark_grey());
                println!("{:40}{}{}", "", "publickey".dark_grey().italic(), " : 40-character CURVE public key in Z85 encoding".dark_grey());
                println!("{:5}{}{}", "", format!("{:35}", "whitelist clear").dark_grey().bold(), "Clear whitelist, meaning all other peers are allowed to subscribe".dark_grey());
            },

            "iomap" => {
                println!("{}{}{}{}", format!("{:62}", "iomap <subcommand> [<parameters>]").dark_grey().bold(), "Execute ".dark_grey(), "subcommand".dark_grey().italic(), " related to IO mapping of integer channels".dark_grey());
                println!("{}", "Available subcommands:".dark_grey());
                println!("{:6}{}{}", "", format!("{:56}", "<in|out> add <start> <length> <interval> <filepath>").dark_grey().bold(), "Add input or output mapping".dark_grey());
                println!("{:62}{}{}", "", "start".dark_grey().italic(), " : unsigned integer in decimal".dark_grey());
                println!("{:62}{}{}", "", "length".dark_grey().italic(), " : positive integer in decimal".dark_grey());
                println!("{:62}{}{}", "", "interval".dark_grey().italic(), " : positive integer in decimal (μs)".dark_grey());
                println!("{:62}{}{}", "", "filepath".dark_grey().italic(), " : string without spaces".dark_grey());
                println!("{:6}{}{}", "", format!("{:56}", "<in|out> del <index>").dark_grey().bold(), "Remove input or output mapping".dark_grey());
                println!("{:62}{}{}", "", "index".dark_grey().italic(), " : unsigned integer in decimal".dark_grey());
                println!("{:6}{}{}", "", format!("{:56}", "<in|out> list").dark_grey().bold(), "Show all input or output mappings".dark_grey());
                println!("{:6}{}{}", "", format!("{:56}", "update").dark_grey().bold(), "Update, i.e. synchronise, input/output integer channels and files".dark_grey());
            },

            "showsizes" => {
                println!("{}{}", format!("{:32}", "showsizes").dark_grey().bold(), "Show constant sizes of some Ælhometta- and Controller-related arrays".dark_grey());
            },

            "sets" | "settings" => {
                println!("{}{}{}{}", format!("{:32}", "settings [setting]").dark_grey().bold(), "Show value(s) of all settings or of ".dark_grey(), "setting".dark_grey().italic(), " only".dark_grey());
                println!("{:32}{}{}", "", "setting".dark_grey().italic(), " : name of required one. None means all".dark_grey());
            },

            "set" => {
                println!("{}{}{}{}{}", format!("{:32}", "set <setting> <value>").dark_grey().bold(), "Set ".dark_grey(), "setting".dark_grey().italic(), " to ".dark_grey(), "value".dark_grey().italic());
                println!("{:32}{}{}", "", "setting".dark_grey().italic(), " : name of setting (case-insensitive)".dark_grey());
                println!("{:32}{}{}", "", "value".dark_grey().italic(), " : corresponding value".dark_grey());
            },
            
            "hist" | "history" => {
                println!("{}{}{}{}", format!("{:32}", "history [limit]").dark_grey().bold(), "Show history of no more than ".dark_grey(), "limit".dark_grey().italic(), " last commands, with timestamps".dark_grey());
                println!("{:32}{}{}", "", "limit".dark_grey().italic(), " : positive integer in decimal, default is full history length".dark_grey());
            },

            _ => {
                return Err(format!("No help on this command"));
            }
        }

        Ok(())
    }

}