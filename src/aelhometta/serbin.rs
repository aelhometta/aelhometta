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

use num_traits::{
    FromPrimitive,
    ToPrimitive
};

use rand::prelude::*;

use std::{
    collections::{
        HashMap,
        HashSet
    },
    fs::File,
    io::{
        BufReader,
        BufWriter,
        Read,
        Write
    }
};

use crate::serbin::{
    ReadBin,
    WriteBin
};

use super::{
    DEFAULT_ÆLHOMETTA_FILENAME,
    Command,
    Content,
    Construction,
    Controller,
    Flags,
    IntegersFileMapping,
    Node,
    OtherPeer,
    Registers,
    Ælhometta
};

const SIGNATURE: &str = "aelhometta";
pub const FORMAT_VERSION: &str = "00001B";
const LOADABLE_FORMATS: [&str; 6] = [
    FORMAT_VERSION,
    "00001A",
    "000019",
    "000018",
    "000017",
    "000016"
];

impl<W: Write> WriteBin<Command> for W {
    fn write_bin(&mut self, command: Command) -> Result<(), String> {
        self.write_bin(command.to_u8().unwrap_or(0))?;
        Ok(())
    }
}

impl<R: Read> ReadBin<Command> for R {
    fn read_bin(&mut self) -> Result<Command, String> {
        let vi: u8 = self.read_bin()?;
        match Command::from_u8(vi) {
            Some(x) => Ok(x),
            _ => Err(format!("Unknown variant '{}' of Command", vi))
        }
    }
}

impl<W: Write> WriteBin<Construction> for W {
    fn write_bin(&mut self, construction: Construction) -> Result<(), String> {
        self.write_bin(construction.to_u8().unwrap_or(0))?;
        Ok(())
    }
}

impl<R: Read> ReadBin<Construction> for R {
    fn read_bin(&mut self) -> Result<Construction, String> {
        let vi: u8 = self.read_bin()?;
        match Construction::from_u8(vi) {
            Some(x) => Ok(x),
            _ => Err(format!("Unknown variant '{}' of Construction", vi))
        }
    }
}

impl<W: Write> WriteBin<Content> for W {
    fn write_bin(&mut self, content: Content) -> Result<(), String> {
        use Content::*;
        match content {
            Space => {
                self.write_bin(0u8)?;
            },
            Branch => {
                self.write_bin(1u8)?;
            },
            Command(command) => {
                self.write_bin(2u8)?;
                self.write_bin(command)?;
            },
            Construction(construction) => {
                self.write_bin(3u8)?;
                self.write_bin(construction)?;
            }            
        }
        Ok(())
    }
}

impl<R: Read> ReadBin<Content> for R {
    fn read_bin(&mut self) -> Result<Content, String> {
        use Content::*;
        let t: u8 = self.read_bin()?;
        match t {
            0 => {
                Ok(Space)
            },
            1 => {
                Ok(Branch)
            },
            2 => {
                Ok(Command(self.read_bin()?))
            },
            3 => {
                Ok(Construction(self.read_bin()?))
            },
            _ => Err(format!("Unknown variant '{}' of Content", t))
        }
    }
}

impl<W: Write> WriteBin<&Node> for W {
    fn write_bin(&mut self, node: &Node) -> Result<(), String> {
        self.write_bin(node.b_content)?;
        self.write_bin(node.b_next)?;
        self.write_bin(node.b_altnext)?;
        Ok(())
    }
}

impl<R: Read> ReadBin<Node> for R {
    fn read_bin(&mut self) -> Result<Node, String> {
        let b_content = self.read_bin()?;
        let b_next = self.read_bin()?;
        let b_altnext = self.read_bin()?;
        Ok(Node {
            b_content,
            b_next,
            b_altnext
        })
    }
}

impl<W: Write> WriteBin<&Registers> for W {
    fn write_bin(&mut self, regs: &Registers) -> Result<(), String> {
        self.write_bin(regs.integer)?;
        Ok(())
    }
}

impl<R: Read> ReadBin<Registers> for R {
    fn read_bin(&mut self) -> Result<Registers, String> {
        let integer = self.read_bin()?;
        Ok(Registers {
            integer
        })
    }
}

impl<W: Write> WriteBin<&Flags> for W {
    fn write_bin(&mut self, flags: &Flags) -> Result<(), String> {
        self.write_bin(flags.success)?;
        Ok(())
    }
}

impl<R: Read> ReadBin<Flags> for R {
    fn read_bin(&mut self) -> Result<Flags, String> {
        let success = self.read_bin()?;
        Ok(Flags {
            success
        })
    }
}

impl<W: Write> WriteBin<&Controller> for W {
    fn write_bin(&mut self, ctrl: &Controller) -> Result<(), String> {
        self.write_bin(ctrl.chain_start_optuid)?;
        self.write_bin(ctrl.exec_optuid)?;
        self.write_bin(ctrl.data_optuids.len())?;
        for opt in & ctrl.data_optuids {
            self.write_bin(*opt)?;
        }        
        self.write_bin(ctrl.i_data_optuid)?;
        self.write_bin(ctrl.new_chain_optuid)?;
        match & ctrl.new_controller {
            Some(boxctrl) => {
                self.write_bin(true)?;
                self.write_bin(boxctrl.as_ref())?;
            },
            None => {
                self.write_bin(false)?;
            }
        }
        self.write_bin(& ctrl.registers)?;
        self.write_bin(& ctrl.flags)?;
        self.write_bin(ctrl.optuids.len())?;
        for opt in & ctrl.optuids {
            self.write_bin(*opt)?;
        }        
        self.write_bin(ctrl.i_optuid)?;
        self.write_bin(ctrl.integers.len())?;
        for int in & ctrl.integers {
            self.write_bin(*int)?;
        }        
        self.write_bin(ctrl.i_integer)?;
        self.write_bin(ctrl.optuid_channels.len())?;
        for c in & ctrl.optuid_channels {
            self.write_bin(*c)?;
        }
        self.write_bin(ctrl.i_optuid_channel)?;
        self.write_bin(ctrl.i_peer)?;
        self.write_bin(ctrl.integer_channels.len())?;
        for c in & ctrl.integer_channels {
            self.write_bin(*c)?;
        }
        self.write_bin(ctrl.i_integer_channel)?;
        self.write_bin(ctrl.generation)?;
        self.write_bin(ctrl.ticks)?;
        Ok(())
    }
}

impl<R: Read> ReadBin<Controller> for R {
    fn read_bin(&mut self) -> Result<Controller, String> {
        let chain_start_optuid = self.read_bin()?;
        let exec_optuid = self.read_bin()?;
        let l: usize = self.read_bin()?;
        let mut data_optuids = Vec::with_capacity(l);
        for _ in 0..l {
            data_optuids.push(self.read_bin()?);
        }
        let i_data_optuid = self.read_bin()?;
        let new_chain_optuid = self.read_bin()?;
        let is_some: bool = self.read_bin()?;
        let new_controller = match is_some {
            false => {
                None
            },
            true => {
                Some(Box::new(self.read_bin()?))
            }
        };
        let registers = self.read_bin()?;
        let flags = self.read_bin()?;
        let l: usize = self.read_bin()?;
        let mut optuids = Vec::with_capacity(l);
        for _ in 0..l {
            optuids.push(self.read_bin()?);
        }
        let i_optuid = self.read_bin()?;
        let l: usize = self.read_bin()?;
        let mut integers = Vec::with_capacity(l);
        for _ in 0..l {
            integers.push(self.read_bin()?);
        }
        let i_integer = self.read_bin()?;
        let l: usize = self.read_bin()?;
        let mut optuid_channels = Vec::with_capacity(l);
        for _ in 0..l {
            optuid_channels.push(self.read_bin()?);
        }
        let i_optuid_channel = self.read_bin()?;
        let i_peer = self.read_bin()?;
        let l: usize = self.read_bin()?;
        let mut integer_channels = Vec::with_capacity(l);
        for _ in 0..l {
            integer_channels.push(self.read_bin()?);
        }
        let i_integer_channel = self.read_bin()?;
        let generation = self.read_bin()?;
        let ticks = self.read_bin()?;

        Ok(Controller {
            chain_start_optuid,
            exec_optuid,
            data_optuids,
            i_data_optuid,
            new_chain_optuid,
            new_controller,
            registers,
            flags,
            optuids,
            i_optuid,
            integers,
            i_integer,
            optuid_channels,
            i_optuid_channel,
            i_peer,
            integer_channels,
            i_integer_channel,
            generation,
            ticks
        })
    }
}

impl<W: Write> WriteBin<&OtherPeer> for W {
    fn write_bin(&mut self, op: &OtherPeer) -> Result<(), String> {
        self.write_bin(op.publickey.as_str())?;
        self.write_bin(op.onion.as_str())?;
        self.write_bin(op.port)?;

        self.write_bin(op.ether_integers.len())?;
        for i in & op.ether_integers {
            self.write_bin(*i)?;
        }

        self.write_bin(op.ut_last_update)?;
        Ok(())
    }
}

impl<R: Read> ReadBin<OtherPeer> for R {
    fn read_bin(&mut self) -> Result<OtherPeer, String> {
        let publickey: String = self.read_bin()?;
        let onion: String = self.read_bin()?;
        let port = self.read_bin()?;

        let l: usize = self.read_bin()?;
        let mut ether_integers = Vec::with_capacity(l);
        for _ in 0..l {
            ether_integers.push(self.read_bin()?);
        }

        let ut_last_update = self.read_bin()?;

        Ok(OtherPeer {
            publickey,
            onion,
            port,
            ether_integers,
            ut_last_update
        })
    }
}

impl<W: Write> WriteBin<&IntegersFileMapping> for W {
    fn write_bin(&mut self, ifm: &IntegersFileMapping) -> Result<(), String> {
        self.write_bin(ifm.start)?;
        self.write_bin(ifm.length)?;
        self.write_bin(ifm.interval)?;
        self.write_bin(ifm.filepath.as_str())?;
        self.write_bin(ifm.ut_last_update)?;    
        Ok(())
    }
}

impl<R: Read> ReadBin<IntegersFileMapping> for R {
    fn read_bin(&mut self) -> Result<IntegersFileMapping, String> {
        let start = self.read_bin()?;
        let length = self.read_bin()?;
        let interval = self.read_bin()?;
        let filepath: String = self.read_bin()?;
        let ut_last_update = self.read_bin()?;

        Ok(IntegersFileMapping {
            start,
            length,
            interval,
            filepath,
            ut_last_update
        })
    }
}

impl<W: Write> WriteBin<&Ælhometta> for W {
    fn write_bin(&mut self, æh: &Ælhometta) -> Result<(), String> {
        // Signature & format version
        for b in SIGNATURE.as_bytes() {
            self.write_bin(*b)?;
        }
        for b in FORMAT_VERSION.as_bytes() {
            self.write_bin(*b)?;
        }

        // Serialisable part
        self.write_bin(æh.max_num_chains_binlog)?;

        self.write_bin(æh.new_node_uid)?;

        self.write_bin(æh.nodes.len())?;
        for (uid, node) in & æh.nodes {
            self.write_bin(*uid)?;
            self.write_bin(node)?;
        }

        self.write_bin(æh.nodes_historing.len())?;
        for opt in & æh.nodes_historing {
            self.write_bin(*opt)?;
        }
        self.write_bin(æh.i_nodes_historing)?;

        self.write_bin(æh.new_controller_uid)?;
        
        self.write_bin(æh.controllers.len())?;
        for (uid, ctrl) in & æh.controllers {
            self.write_bin(*uid)?;
            self.write_bin(ctrl)?;
        }

        self.write_bin(æh.controllers_historing.len())?;
        for opt in & æh.controllers_historing {
            self.write_bin(*opt)?;
        }
        self.write_bin(æh.i_controllers_historing)?;

        self.write_bin(æh.commandswitch)?;

        self.write_bin(æh.ether_optuids.len())?;
        for opt in & æh.ether_optuids {
            self.write_bin(*opt)?;
        }

        self.write_bin(æh.ether_integers.len())?;
        for opt in & æh.ether_integers {
            self.write_bin(*opt)?;
        }

        self.write_bin(æh.age)?;

        self.write_bin(æh.spaces_count)?;
        self.write_bin(æh.branches_main_count)?;
        self.write_bin(æh.branches_alt_count)?;

        self.write_bin(æh.commands_count.len())?;
        for (command, count) in & æh.commands_count {
            self.write_bin(*command)?;
            self.write_bin(*count)?;
        }

        self.write_bin(æh.constructions_count.len())?;
        for (construction, count) in & æh.constructions_count {
            self.write_bin(*construction)?;
            self.write_bin(*count)?;
        }

        self.write_bin(æh.glitch_background_prob)?;
        self.write_bin(æh.glitch_background_count)?;

        self.write_bin(æh.glitch_replicate_prob)?;
        self.write_bin(æh.glitch_replicate_count)?;

        self.write_bin(æh.glitch_construct_prob)?;
        self.write_bin(æh.glitch_construct_count)?;

        self.write_bin(æh.share_size)?;
        self.write_bin(æh.share_interval)?;

        self.write_bin(æh.ut_last_share)?;

        self.write_bin(æh.secretkey.as_str())?;
        self.write_bin(æh.port)?;
        self.write_bin(æh.torproxy_port)?;
        self.write_bin(æh.torproxy_host.as_str())?;

        self.write_bin(æh.exposed)?;

        self.write_bin(æh.other_peers.len())?;
        for op in & æh.other_peers {
            self.write_bin(op)?;
        }

        self.write_bin(æh.whitelist.len())?;
        for pk in & æh.whitelist {
            self.write_bin(pk.as_str())?;
        }

        // "before" + "now"
        let mut n = æh.in_permitted_before_num;
        if let Some(ref efunguz) = æh.efunguz {
            n += efunguz.in_permitted_num();
        }
        self.write_bin(n)?;

        // "before" + "now"
        let mut n = æh.in_attempted_before_num;
        if let Some(ref efunguz) = æh.efunguz {
            n += efunguz.in_attempted_num();
        }
        self.write_bin(n)?;

        self.write_bin(æh.output_mappings.len())?;
        for ifm in & æh.output_mappings {
            self.write_bin(ifm)?;
        }

        self.write_bin(æh.input_mappings.len())?;
        for ifm in & æh.input_mappings {
            self.write_bin(ifm)?;
        }

        Ok(())
    }
}

impl<R: Read> ReadBin<Ælhometta> for R {
    fn read_bin(&mut self) -> Result<Ælhometta, String> {
        // Check signature & format version
        let mut strbuf = [0u8; SIGNATURE.len()];
        self.read_exact(&mut strbuf).map_err(|e| e.to_string())?;
        let signature = String::from_utf8_lossy(&strbuf);
        if signature != SIGNATURE {
            return Err(format!("Wrong signature: expected '{}', found '{}'", SIGNATURE, signature));
        }

        let mut strbuf = [0u8; FORMAT_VERSION.len()];
        self.read_exact(&mut strbuf).map_err(|e| e.to_string())?;
        let file_format_version = String::from_utf8_lossy(&strbuf);
        let format_loadable = LOADABLE_FORMATS.contains(& file_format_version.as_ref());
        if !format_loadable {
            return Err(format!("Format not loadable: '{}' (oldest loadable is '{}', current is '{}')", &file_format_version, LOADABLE_FORMATS[0], FORMAT_VERSION));
        } 

        // Serialisable part
        let max_num_chains_binlog: u8 = self.read_bin()?;

        let new_node_uid = self.read_bin()?;
        let l: usize = self.read_bin()?;
        let mut nodes = HashMap::with_capacity(l);
        match file_format_version.as_ref() { // example of conversion between Command encoding in different formats
            FORMAT_VERSION | "00001A" | "000019" => {
                for _ in 0..l {
                    let uid = self.read_bin()?;
                    let node = self.read_bin()?;
                    nodes.insert(uid, node);
                }
            },
            _ => {
                for _ in 0..l {
                    let uid = self.read_bin()?;
                    let mut node: Node = self.read_bin()?;
                    node.b_content = match node.b_content {
                        18 => 19, // IntegerToIntegerIndex was 18, since "000019" is 19
                        19 => 18, // IntegerToIntegerChannel was 19, since "000019" is 18
                        61 => 62, // ShiftUp was 61, since "000019" is 62
                        62 => 61, // ShiftDown was 62, since "000019" is 61
                        _ => node.b_content
                    };
                    nodes.insert(uid, node);
                }
            }
        }

        let l: usize = self.read_bin()?;
        let mut nodes_historing = Vec::with_capacity(l);
        for _ in 0..l {
            nodes_historing.push(self.read_bin()?);
        }
        let i_nodes_historing = self.read_bin()?;

        let new_controller_uid = self.read_bin()?;
        let l: usize = self.read_bin()?;
        let mut controllers = HashMap::with_capacity(l);
        for _ in 0..l {
            let uid = self.read_bin()?;
            let ctrl = self.read_bin()?;
            controllers.insert(uid, ctrl);
        }
        let l: usize = self.read_bin()?;
        let mut controllers_historing = Vec::with_capacity(l);
        for _ in 0..l {
            controllers_historing.push(self.read_bin()?);
        }
        let i_controllers_historing = self.read_bin()?;

        let mut commandswitch = u128::MAX;
        match file_format_version.as_ref() {
            FORMAT_VERSION => {
                commandswitch = self.read_bin()?;
            },
            "00001A" | "000019" | "000018" | "000017" => {
                let introspection: bool = self.read_bin()?;
                if !introspection {
                    commandswitch ^= 1 << Command::GetExecFromOptuid.to_u8().unwrap_or(0);
                    commandswitch ^= 1 << Command::SetOptuidFromExec.to_u8().unwrap_or(0);
                }
            },
            _ => {}
        };

        let l: usize = self.read_bin()?;
        let mut ether_optuids = Vec::with_capacity(l);
        for _ in 0..l {
            ether_optuids.push(self.read_bin()?);
        }

        let l: usize = self.read_bin()?;
        let mut ether_integers = Vec::with_capacity(l);
        for _ in 0..l {
            ether_integers.push(self.read_bin()?);
        }

        let age = self.read_bin()?;

        let spaces_count = match file_format_version.as_ref() {
            FORMAT_VERSION | "00001A" | "000019" | "000018" => self.read_bin()?,
            _ => 0
        };

        let branches_main_count = match file_format_version.as_ref() {
            FORMAT_VERSION | "00001A" | "000019" | "000018" => self.read_bin()?,
            _ => 0
        };

        let branches_alt_count = match file_format_version.as_ref() {
            FORMAT_VERSION | "00001A" | "000019" | "000018" => self.read_bin()?,
            _ => 0
        };

        let l: usize = self.read_bin()?;
        let mut commands_count = HashMap::with_capacity(l);
        for _ in 0..l {
            let command = self.read_bin()?;
            let count = self.read_bin()?;
            commands_count.insert(command, count);
        }

        let constructions_count = match file_format_version.as_ref() {
            FORMAT_VERSION | "00001A" | "000019" | "000018" => {
                let l: usize = self.read_bin()?;
                let mut cons_count = HashMap::with_capacity(l);
                for _ in 0..l {
                    let cons = self.read_bin()?;
                    let count = self.read_bin()?;
                    cons_count.insert(cons, count);
                }
                cons_count
            },
            _ => super::new_constructions_count()
        };

        let glitch_background_prob = self.read_bin()?;
        let glitch_background_count = self.read_bin()?;

        let glitch_replicate_prob = self.read_bin()?;
        let glitch_replicate_count = self.read_bin()?;

        let glitch_construct_prob = self.read_bin()?;
        let glitch_construct_count = self.read_bin()?;
        
        let share_size = self.read_bin()?;
        let share_interval = self.read_bin()?;

        let ut_last_share = self.read_bin()?;

        let secretkey: String = self.read_bin()?;
        let port = self.read_bin()?;
        let torproxy_port = self.read_bin()?;
        let torproxy_host: String = self.read_bin()?;

        let exposed = self.read_bin()?;

        let l: usize = self.read_bin()?;
        let mut other_peers = Vec::<OtherPeer>::with_capacity(l);
        for _ in 0..l {
            other_peers.push(self.read_bin()?);
        }

        let l: usize = self.read_bin()?;
        let mut whitelist = HashSet::new();
        for _ in 0..l {
            whitelist.insert(self.read_bin()?);
        }

        let in_permitted_before_num = match file_format_version.as_ref() {
            FORMAT_VERSION | "00001A" => self.read_bin()?,
            _ => 0
        };

        let in_attempted_before_num = match file_format_version.as_ref() {
            FORMAT_VERSION | "00001A" => self.read_bin()?,
            _ => 0
        };

        let l: usize = self.read_bin()?;
        let mut output_mappings = Vec::<IntegersFileMapping>::with_capacity(l);
        for _ in 0..l {
            output_mappings.push(self.read_bin()?);
        }

        let l: usize = self.read_bin()?;
        let mut input_mappings = Vec::<IntegersFileMapping>::with_capacity(l);
        for _ in 0..l {
            input_mappings.push(self.read_bin()?);
        }

        // Non-serialisable part

        let max_num_chains: usize = 1 <<  max_num_chains_binlog;
        let max_num_chains_binmask: usize = max_num_chains - 1;
        let rng = thread_rng();
        let efunguz = None;

        let mut æh = Ælhometta {
            max_num_chains_binlog,
            new_node_uid,
            nodes,
            nodes_historing,
            i_nodes_historing,
            new_controller_uid,
            controllers,
            controllers_historing,
            i_controllers_historing,
            commandswitch,
            ether_optuids,
            ether_integers,
            age,
            spaces_count,
            branches_main_count,
            branches_alt_count,
            commands_count,
            constructions_count,
            glitch_background_prob,
            glitch_background_count,
            glitch_replicate_prob,
            glitch_replicate_count,
            glitch_construct_prob,
            glitch_construct_count,
            share_size,
            share_interval,
            ut_last_share,
            secretkey,
            port,
            torproxy_port,
            torproxy_host,
            exposed,
            other_peers,
            whitelist,
            in_permitted_before_num,
            in_attempted_before_num,
            output_mappings,
            input_mappings,

            max_num_chains,
            max_num_chains_binmask,
            rng,
            efunguz
        };

        if exposed {
            æh.peer_expose()?;
        }

        Ok(æh)
    }
}

impl Ælhometta {
    pub fn save(&self, filepath: &str) -> Result<(), String> {
        // "Binary serialization", cf. load()
        let mut writer = BufWriter::new(File::create(filepath).map_err(|err| format!("Cannot create '{}': {}", filepath, &err))?);
        writer.write_bin(self).map_err(|err| format!("Cannot write to '{}': {}", filepath, &err))?;
        writer.flush().map_err(|err| format!("Cannot flush: {}", &err))?;
        Ok(())
    }

    pub fn save_default(&self) -> Result<(), String> {
        self.save(DEFAULT_ÆLHOMETTA_FILENAME)
    }

    pub fn load(filepath: &str) -> Result<Self, String> {
        // "Binary deserialization", cf. save()
        let mut reader = BufReader::new(File::open(filepath).map_err(|err| format!("Cannot open '{}': {}", filepath, &err))?);
        reader.read_bin().map_err(|err| format!("Cannot read from '{}': {}", filepath, &err))
    }

    pub fn load_default() -> Result<Self, String> {
        Self::load(DEFAULT_ÆLHOMETTA_FILENAME)
    }

}
