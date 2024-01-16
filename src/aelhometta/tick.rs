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

use rand::prelude::*;

use std::collections::HashSet;

use crate::serbin::{
    OtBits,
    ToBits
};

use super::{
    Integer,
    Uid,
    CONTENTS,
    Command,
    Content,
    Construction,
    Optuid,
    Ælhometta,
    Controller,
    Flags,
    Registers,
    TickData
};

impl Ælhometta {
    pub fn tick(&mut self, ctrl_optuid: &Optuid) -> TickData {
        self.iomap_update();

        self.peer_update();

        let mut tick_data = TickData::new_default();

        if self.controllers.len() > 0 {
            // Choose given controller, or random one if none has been given, and work with its copy
            let cuid = match ctrl_optuid {
                &Some(ctrl_uid) => ctrl_uid,
                &None => {
                    let (cuid, _) = self.controllers.iter().choose(&mut self.rng).unwrap(); // should not panic, because self.controllers isn't empty
                    *cuid
                }
            };

            if let Some(ctrl) = self.controllers.get(&cuid) {
                let mut ctrl = ctrl.clone();

                tick_data.controller_optuid = Some(cuid);
                tick_data.exec_optuid = ctrl.exec_optuid;

                // Obtain node from exec pointkey
                if let Some(enuid) = ctrl.exec_optuid {
                    if self.nodes.contains_key(&enuid) {
                        let enode = self.nodes.get(&enuid).cloned().unwrap();
                        let econtent = Content::ot_bits(enode.b_content);

                        tick_data.exec_optcontent = Some(econtent);

                        let mut next_exec_forced = false;

                        // Execute command, ignore other types at execution
                        match econtent {
                            Content::Command(command) => {
                                use self::Command::*;
                                ctrl.flags.success = false; // "pessimistic default"
                                match command {
                                    Abs => {
                                        ctrl.registers.integer = ctrl.registers.integer.abs();
                                        ctrl.flags.success = true;
                                    },

                                    Add => {
                                        let r = (ctrl.registers.integer as i128) + (ctrl.integers[ctrl.i_integer] as i128);
                                        ctrl.flags.success = (r >= (i64::MIN as i128)) && (r <= (i64::MAX as i128));
                                        ctrl.registers.integer = r as i64;
                                    },

                                    BitAnd => {
                                        ctrl.registers.integer &= ctrl.integers[ctrl.i_integer];
                                        ctrl.flags.success = true;
                                    },

                                    BitNot => {
                                        ctrl.registers.integer = ! ctrl.registers.integer;
                                        ctrl.flags.success = true;
                                    },

                                    BitOr => {
                                        ctrl.registers.integer |= ctrl.integers[ctrl.i_integer];
                                        ctrl.flags.success = true;
                                    },

                                    BitXor => {
                                        ctrl.registers.integer ^= ctrl.integers[ctrl.i_integer];
                                        ctrl.flags.success = true;
                                    },

                                    Construct => {
                                        use self::Construction::*;
                                        if ctrl.new_chain_optuid.is_some() && ctrl.new_controller.is_some() { // in "NewChain, active" mode now
                                            let mut readuids_set: HashSet<Uid> = HashSet::new();
                                            let mut newuids_set: HashSet<Uid> = HashSet::new();
                                            let mut constr_uids_stack: Vec<Uid> = Vec::new();
                                            let mut constr_alt_next: bool = false;
                                            loop {
                                                if let Some(rnuid) = ctrl.data_optuids[ctrl.i_data_optuid] {
                                                    if self.nodes.contains_key(&rnuid) {
                                                        if ! readuids_set.contains(&rnuid) { // stop if loop occurs
                                                            readuids_set.insert(rnuid);
                                                            if ! newuids_set.contains(&rnuid) { // stop if reading what has been written by this very Construct (another kind of loop)
                                                                if let Some(wnuid) = ctrl.new_chain_optuid {
                                                                    if self.nodes.contains_key(&wnuid) {
                                                                        let rnode = self.nodes.get(&rnuid).unwrap();                                                                    
                                                                        ctrl.data_optuids[ctrl.i_data_optuid] = Optuid::ot_bits(rnode.b_next);

                                                                        let mut rcontent = Content::ot_bits(rnode.b_content);
                                                                        if self.rng.gen_bool(self.glitch_construct_prob) {
                                                                            rcontent = CONTENTS[self.rng.gen_range(0..CONTENTS.len())];
                                                                            self.glitch_construct_count += 1;
                                                                        }

                                                                        let wcontent = Content::ot_bits(self.nodes.get(&wnuid).unwrap().b_content);
                                                                        match rcontent {
                                                                            Content::Space | Content::Branch | Content::Command(_) => {
                                                                                let nouid = self.add_new_node(rcontent);
                                                                                if let Some(nuid) = nouid {
                                                                                    newuids_set.insert(nuid);
                                                                                }
                                                                                if let Some(wnode) = self.nodes.get_mut(&wnuid) {
                                                                                    match wcontent {
                                                                                        Content::Branch => {
                                                                                            if constr_alt_next {
                                                                                                wnode.b_altnext = nouid.to_bits();
                                                                                                constr_alt_next = false;
                                                                                            } else {
                                                                                                wnode.b_next = nouid.to_bits();
                                                                                            }
                                                                                        },
                                                                                        _ => {
                                                                                            wnode.b_next = nouid.to_bits();
                                                                                        }
                                                                                    }
                                                                                    ctrl.new_chain_optuid = nouid;
                                                                                } else { break; }
                                                                            },
                                                                            Content::Construction(construction) => {
                                                                                match construction {
                                                                                    AltNext => {
                                                                                        constr_alt_next = true;
                                                                                    },
                                                                                    Discard => {
                                                                                        constr_uids_stack.pop();
                                                                                    },
                                                                                    NextToStored => {
                                                                                        if constr_uids_stack.len() != 0 {
                                                                                            let st_ouid = constr_uids_stack.last().cloned();
                                                                                            if let Some(wnode) = self.nodes.get_mut(&wnuid) {
                                                                                                match wcontent {
                                                                                                    Content::Branch => {
                                                                                                        if constr_alt_next {
                                                                                                            wnode.b_altnext = st_ouid.to_bits();
                                                                                                            constr_alt_next = false;
                                                                                                        } else {
                                                                                                            wnode.b_next = st_ouid.to_bits();
                                                                                                        }
                                                                                                    },
                                                                                                    _ => {
                                                                                                        wnode.b_next = st_ouid.to_bits();
                                                                                                    }
                                                                                                }
                                                                                            } else { break; }
                                                                                        }
                                                                                    },
                                                                                    Restore => {
                                                                                        if constr_uids_stack.len() != 0 {
                                                                                            ctrl.new_chain_optuid = constr_uids_stack.last().cloned();
                                                                                        }
                                                                                    },
                                                                                    Store => {
                                                                                        constr_uids_stack.push(wnuid);
                                                                                    },
                                                                                    Swap => {
                                                                                        let l = constr_uids_stack.len();
                                                                                        if l >= 2 {
                                                                                            constr_uids_stack.swap(l - 1, l - 2);
                                                                                        }
                                                                                    },
                                                                                    Terminus => {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                            }
                                                                        };
                                                                            
                                                                    } else { break; }
                                                                } else { break; }
                                                            } else { break; }
                                                        } else { break; }
                                                    } else { break; }
                                                } else { break; }
                                            }
                                            ctrl.flags.success = true;
                                        }                                  
                                    },

                                    DataOptuidIndexToInteger => {
                                        ctrl.registers.integer = ctrl.i_data_optuid as Integer;
                                        ctrl.flags.success = true;
                                    },

                                    Decrement => {
                                        ctrl.registers.integer -= 1;
                                        ctrl.flags.success = true;
                                    },

                                    Divide => {
                                        if ctrl.integers[ctrl.i_integer] != 0 {
                                            let r = (ctrl.registers.integer as i128) / (ctrl.integers[ctrl.i_integer] as i128);
                                            ctrl.flags.success = (r >= (i64::MIN as i128)) && (r <= (i64::MAX as i128));
                                            ctrl.registers.integer = r as i64;
                                        }
                                    },

                                    GetExecFromOptuid => { // unique in making "next exec forced" true
                                        if self.introspection {
                                            ctrl.exec_optuid = ctrl.optuids[ctrl.i_optuid];
                                            next_exec_forced = true;
                                        } // else same as Space / NOP
                                        ctrl.flags.success = true;
                                    },

                                    GetIntegerFromIntegers => {
                                        ctrl.registers.integer = ctrl.integers[ctrl.i_integer];
                                        ctrl.flags.success = true;
                                    },

                                    Increment => {
                                        ctrl.registers.integer += 1;
                                        ctrl.flags.success = true;
                                    },

                                    Insert => {
                                        if let Some(wnuid) = ctrl.data_optuids[ctrl.i_data_optuid] {
                                            if self.nodes.contains_key(&wnuid) {
                                                let wn_next_optuid = Optuid::ot_bits(self.nodes.get(&wnuid).unwrap().b_next);
                                                let nnouid = self.add_new_node_to_existing(Content::ot_bits(ctrl.registers.integer as u8), & ctrl.data_optuids[ctrl.i_data_optuid]);
                                                if let Some(nnuid) = nnouid {                                                
                                                    if let Some(nnode) = self.nodes.get_mut(&nnuid) {
                                                        nnode.b_next = wn_next_optuid.to_bits();
                                                        ctrl.data_optuids[ctrl.i_data_optuid] = nnouid;
                                                        ctrl.flags.success = true;
                                                    }
                                                }
                                            }
                                        }
                                    },

                                    IntegerChannelToInteger => {
                                        ctrl.registers.integer = ctrl.integer_channels[ctrl.i_integer_channel] as Integer;
                                        ctrl.flags.success = true;
                                    },

                                    IntegerIndexToInteger => {
                                        ctrl.registers.integer = ctrl.i_integer as Integer;
                                        ctrl.flags.success = true;
                                    },

                                    IntegerToDataOptuidIndex => {
                                        if (ctrl.registers.integer >= 0) && ((ctrl.registers.integer as usize) < ctrl.data_optuids.len()) {
                                            ctrl.i_data_optuid = ctrl.registers.integer as usize;
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    IntegerToIntegerChannel => {
                                        if ctrl.registers.integer >= 0 {
                                            ctrl.integer_channels[ctrl.i_integer_channel] = ctrl.registers.integer as usize;
                                            ctrl.flags.success = true;
                                        }                                        
                                    },

                                    IntegerToIntegerIndex => {
                                        if (ctrl.registers.integer >= 0) && ((ctrl.registers.integer as usize) < ctrl.integers.len()) {
                                            ctrl.i_integer = ctrl.registers.integer as usize;
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    IntegerToOptuidChannel => {
                                        if (ctrl.registers.integer >= 0) && ((ctrl.registers.integer as usize) < self.ether_optuids.len()) {
                                            ctrl.optuid_channels[ctrl.i_optuid_channel] = ctrl.registers.integer as usize;
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    IntegerToOptuidIndex => {
                                        if (ctrl.registers.integer >= 0) && ((ctrl.registers.integer as usize) < ctrl.optuids.len()) {
                                            ctrl.i_optuid = ctrl.registers.integer as usize;
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    IntegerToPeer => {
                                        if (ctrl.registers.integer >= 0) && ((ctrl.registers.integer as usize) <= self.other_peers.len()) {
                                            ctrl.i_peer = ctrl.registers.integer as usize;
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    IntegerToSuccess => {
                                        ctrl.flags.success = (ctrl.registers.integer & 1) != 0;
                                    },

                                    Multiply => {
                                        let r = (ctrl.registers.integer as i128) * (ctrl.integers[ctrl.i_integer] as i128);
                                        ctrl.flags.success = (r >= (i64::MIN as i128)) && (r <= (i64::MAX as i128));
                                        ctrl.registers.integer = r as i64;
                                    },

                                    Negate => {
                                        let r = -(ctrl.registers.integer as i128);
                                        ctrl.flags.success = (r >= (i64::MIN as i128)) && (r <= (i64::MAX as i128));
                                        ctrl.registers.integer = r as i64;
                                    },

                                    NewChainAddInteger => {
                                        if let Some(ref mut nctrl) = ctrl.new_controller {
                                            nctrl.integers[nctrl.i_integer] = ctrl.registers.integer;
                                            nctrl.i_integer = (nctrl.i_integer + 1) % nctrl.integers.len();
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    NewChainAddIntegerChannel => {
                                        if let Some(ref mut nctrl) = ctrl.new_controller {
                                            nctrl.integer_channels[nctrl.i_integer_channel] = ctrl.integer_channels[ctrl.i_integer_channel];
                                            nctrl.i_integer_channel = (nctrl.i_integer_channel + 1) % nctrl.integer_channels.len();
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    NewChainAddOptuid => {
                                        if let Some(ref mut nctrl) = ctrl.new_controller {
                                            nctrl.optuids[nctrl.i_optuid] = ctrl.optuids[ctrl.i_optuid];
                                            nctrl.i_optuid = (nctrl.i_optuid + 1) % nctrl.optuids.len();
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    NewChainAddOptuidChannel => {
                                        if let Some(ref mut nctrl) = ctrl.new_controller {
                                            nctrl.optuid_channels[nctrl.i_optuid_channel] = ctrl.optuid_channels[ctrl.i_optuid_channel];
                                            nctrl.i_optuid_channel = (nctrl.i_optuid_channel + 1) % nctrl.optuid_channels.len();
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    NewChainDetach => {
                                        if ctrl.new_chain_optuid.is_some() {
                                            ctrl.new_chain_optuid = None;
                                            if ctrl.new_controller.is_some() { // active chain (with controller)
                                                let nctrl = ctrl.new_controller.as_mut().unwrap();
                                                nctrl.exec_optuid = nctrl.chain_start_optuid;
                                                nctrl.i_optuid = 0;
                                                nctrl.i_integer = 0;
                                                nctrl.i_optuid_channel = 0;
                                                nctrl.i_integer_channel = 0;
                                                nctrl.generation = ctrl.generation + 1;
                                                self.add_controller(nctrl.as_ref().clone());
                                                ctrl.new_controller = None;
                                            }
                                            ctrl.flags.success = true;    
                                        }
                                    },

                                    NewChainInitActive => {
                                        let nnouid = self.add_new_node(Content::Space);
                                        if let Some(_) = nnouid {
                                            ctrl.new_chain_optuid = nnouid;
                                            ctrl.optuids[ctrl.i_optuid] = nnouid;
                                            ctrl.new_controller = Some(Box::new(Controller::new()));
                                            ctrl.new_controller.as_mut().unwrap().chain_start_optuid = nnouid;
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    NewChainInitPassive => {
                                        let nnouid = self.add_new_node(Content::Space);
                                        if let Some(_) = nnouid {
                                            ctrl.new_chain_optuid = nnouid;
                                            ctrl.optuids[ctrl.i_optuid] = nnouid;
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    NextDataOptuid => {
                                        ctrl.i_data_optuid += 1;
                                        if ctrl.i_data_optuid < ctrl.data_optuids.len() {
                                            ctrl.flags.success = true;
                                        } else {
                                            ctrl.i_data_optuid -= 1;
                                        }
                                    },

                                    NextInteger => {
                                        ctrl.i_integer +=  1;
                                        if ctrl.i_integer < ctrl.integers.len() {
                                            ctrl.flags.success = true;
                                        } else {
                                            ctrl.i_integer -= 1;
                                        }                                        
                                    },

                                    NextIntegerChannel => {
                                        ctrl.i_integer_channel += 1;
                                        if ctrl.i_integer_channel < ctrl.integer_channels.len() {
                                            ctrl.flags.success = true;
                                        } else {
                                            ctrl.i_integer_channel -= 1;
                                        }
                                    },

                                    NextOptuid => {
                                        ctrl.i_optuid += 1;
                                        if ctrl.i_optuid < ctrl.optuids.len() {
                                            ctrl.flags.success = true;
                                        } else {
                                            ctrl.i_optuid -= 1;
                                        }
                                    },

                                    NextOptuidChannel => {
                                        ctrl.i_optuid_channel += 1;
                                        if ctrl.i_optuid_channel < ctrl.optuid_channels.len() {
                                            ctrl.flags.success = true;
                                        } else {
                                            ctrl.i_optuid_channel -= 1;
                                        }                                        
                                    },

                                    NextPeer => {
                                        ctrl.i_peer += 1;
                                        if ctrl.i_peer <= self.other_peers.len() { // 0th is "this", +1
                                            ctrl.flags.success = true;
                                        } else {
                                            ctrl.i_peer -= 1;
                                        }
                                    },

                                    OptuidChannelToInteger => {
                                        ctrl.registers.integer = ctrl.optuid_channels[ctrl.i_optuid_channel] as Integer;
                                        ctrl.flags.success = true;
                                    },

                                    OptuidIndexToInteger => {
                                        ctrl.registers.integer = ctrl.i_optuid as Integer;
                                        ctrl.flags.success = true;
                                    },

                                    PeerToInteger => {
                                        ctrl.registers.integer = ctrl.i_peer as Integer;
                                        ctrl.flags.success = true;
                                    },

                                    PreviousDataOptuid => {
                                        if ctrl.i_data_optuid > 0 {
                                            ctrl.i_data_optuid -= 1;
                                            ctrl.flags.success = true;
                                        }                                        
                                    },

                                    PreviousInteger => {
                                        if ctrl.i_integer > 0 {
                                            ctrl.i_integer -= 1;
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    PreviousIntegerChannel => {
                                        if ctrl.i_integer_channel > 0 {
                                            ctrl.i_integer_channel -= 1;
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    PreviousOptuid => {
                                        if ctrl.i_optuid > 0 {
                                            ctrl.i_optuid -= 1;
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    PreviousOptuidChannel => {
                                        if ctrl.i_optuid_channel > 0 {
                                            ctrl.i_optuid_channel -= 1;
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    PreviousPeer => {
                                        if ctrl.i_peer > 0 {
                                            ctrl.i_peer -= 1;
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    RandomContent => {
                                        ctrl.registers.integer = CONTENTS[self.rng.gen_range(0..CONTENTS.len())].to_bits() as Integer;
                                        ctrl.flags.success = true;
                                    },

                                    RandomInteger => {
                                        ctrl.registers.integer = self.rng.gen::<Integer>();
                                        ctrl.flags.success = true;
                                    },

                                    Read => {
                                        if let Some(rnuid) = ctrl.data_optuids[ctrl.i_data_optuid] {
                                            self.nodes.entry(rnuid).and_modify(|rnode| {
                                                ctrl.registers.integer = rnode.b_content as Integer;
                                                ctrl.data_optuids[ctrl.i_data_optuid] = Optuid::ot_bits(rnode.b_next);
                                                ctrl.flags.success = true;
                                            });
                                        }
                                    },

                                    ReceiveInteger => {
                                        let chan = ctrl.integer_channels[ctrl.i_integer_channel];
                                        if ctrl.i_peer == 0 { // this peer
                                            if chan < self.ether_integers.len() {
                                                ctrl.registers.integer = self.ether_integers[chan];
                                                ctrl.flags.success = true;
                                            }
                                        } else if ctrl.i_peer <= self.other_peers.len() {
                                            if chan < self.other_peers[ctrl.i_peer - 1].ether_integers.len() {
                                                ctrl.registers.integer = self.other_peers[ctrl.i_peer - 1].ether_integers[chan];
                                                ctrl.flags.success = true;
                                            }
                                        }                                        
                                    },

                                    ReceiveOptuid => {
                                        if let Some(uid) = self.ether_optuids[ctrl.optuid_channels[ctrl.i_optuid_channel]] {
                                            ctrl.optuids[ctrl.i_optuid] = Some(uid);
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    Remainder => {
                                        if ctrl.integers[ctrl.i_integer] != 0 {
                                            ctrl.registers.integer %= ctrl.integers[ctrl.i_integer];
                                            ctrl.flags.success = true;
                                        }
                                    },

                                    Remove => {
                                        if let Some(rnuid) = ctrl.data_optuids[ctrl.i_data_optuid] {
                                            if self.nodes.contains_key(&rnuid) {
                                                let r_next = Optuid::ot_bits(self.nodes.get(&rnuid).unwrap().b_next);
                                                self.remove_node(& ctrl.data_optuids[ctrl.i_data_optuid]);
                                                ctrl.data_optuids[ctrl.i_data_optuid] = r_next;
                                                ctrl.flags.success = false;
                                            }
                                        }
                                    },

                                    Replicate => {
                                        if ctrl.new_chain_optuid.is_some() && ctrl.new_controller.is_none() { // in "NewChain, passive" mode now
                                            let mut readuids_set: HashSet<Uid> = HashSet::new();
                                            let mut newuids_set: HashSet<Uid> = HashSet::new();
                                            loop {
                                                if let Some(rnuid) = ctrl.data_optuids[ctrl.i_data_optuid] {
                                                    if self.nodes.contains_key(&rnuid) {
                                                        if ! readuids_set.contains(&rnuid) { // stop if loop occurs
                                                            readuids_set.insert(rnuid);
                                                            if ! newuids_set.contains(&rnuid) { // stop if reading what has been written by this very Replicate (another kind of loop)
                                                                if let Some(wnuid) = ctrl.new_chain_optuid {
                                                                    if self.nodes.contains_key(&wnuid) {
                                                                        let rnode = self.nodes.get(&rnuid).unwrap();
                                                                        ctrl.data_optuids[ctrl.i_data_optuid] = Optuid::ot_bits(rnode.b_next);

                                                                        let mut rw_content = Content::ot_bits(rnode.b_content);
                                                                        if self.rng.gen_bool(self.glitch_replicate_prob) {
                                                                            rw_content = CONTENTS[self.rng.gen_range(0..CONTENTS.len())];
                                                                            self.glitch_replicate_count += 1;
                                                                        }

                                                                        let nouid = self.add_new_node(rw_content);

                                                                        if let Some(nuid) = nouid {
                                                                            newuids_set.insert(nuid);
                                                                        }
                                                                        if let Some(wnode) = self.nodes.get_mut(&wnuid) {
                                                                            wnode.b_next = nouid.to_bits();
                                                                            ctrl.new_chain_optuid = nouid;
                                                                        } else { break; }                                                                  
                                                                    } else { break; }
                                                                } else { break; }
                                                            } else { break; }
                                                        } else { break; }
                                                    } else { break; }
                                                } else { break; }
                                            }
                                            ctrl.flags.success = true;
                                        }                                    
                                    },

                                    Restart => {
                                        ctrl.exec_optuid = ctrl.chain_start_optuid;
                                        ctrl.i_optuid = 0;
                                        ctrl.i_data_optuid = 0;
                                        ctrl.i_integer = 0;
                                        ctrl.i_optuid_channel = 0;
                                        ctrl.i_peer = 0;
                                        ctrl.i_integer_channel = 0;
                                        ctrl.new_chain_optuid = None;
                                        ctrl.new_controller = None;
                                        ctrl.registers = Registers::new();
                                        ctrl.flags = Flags::new();
                                    },

                                    SetDataOptuidFromOptuid => {
                                        ctrl.data_optuids[ctrl.i_data_optuid] = ctrl.optuids[ctrl.i_optuid];
                                        ctrl.flags.success = true;
                                    },

                                    SetIntegersFromInteger => {
                                        ctrl.integers[ctrl.i_integer] = ctrl.registers.integer;
                                        ctrl.flags.success = true;
                                    },

                                    SetOptuidFromDataOptuid => {
                                        ctrl.optuids[ctrl.i_optuid] = ctrl.data_optuids[ctrl.i_data_optuid];
                                        ctrl.flags.success = true;
                                    },

                                    SetOptuidFromExec => {
                                        if self.introspection {
                                            ctrl.optuids[ctrl.i_optuid] = ctrl.exec_optuid;
                                        } // else same as Space / NOP
                                        ctrl.flags.success = true;
                                    },

                                    ShiftUp => {
                                        ctrl.registers.integer <<= 1;
                                        ctrl.flags.success = true;
                                    },

                                    ShiftDown => {
                                        ctrl.registers.integer >>= 1;
                                        ctrl.flags.success = true;
                                    },

                                    Sign => {
                                        ctrl.registers.integer = ctrl.registers.integer.signum();
                                        ctrl.flags.success = true;
                                    },

                                    Skip => {
                                        if let Some(snuid) = ctrl.data_optuids[ctrl.i_data_optuid] {
                                            self.nodes.entry(snuid).and_modify(|snode| {
                                                ctrl.data_optuids[ctrl.i_data_optuid] = Optuid::ot_bits(snode.b_next);
                                                ctrl.flags.success = true;
                                            });
                                        }
                                    },

                                    Square => {
                                        let r = (ctrl.registers.integer as i128) * (ctrl.registers.integer as i128);
                                        ctrl.flags.success = (r >= (i64::MIN as i128)) && (r <= (i64::MAX as i128));
                                        ctrl.registers.integer = r as i64;
                                    },

                                    Subtract => {
                                        let r = (ctrl.registers.integer as i128) - (ctrl.integers[ctrl.i_integer] as i128);
                                        ctrl.flags.success = (r >= (i64::MIN as i128)) && (r <= (i64::MAX as i128));
                                        ctrl.registers.integer = r as i64;
                                    },

                                    SuccessToInteger => {
                                        ctrl.registers.integer = ctrl.flags.success as Integer;
                                        ctrl.flags.success = true;
                                    },

                                    TestDataOptuid => {
                                        if let Some(rnuid) = ctrl.data_optuids[ctrl.i_data_optuid] {
                                            ctrl.flags.success = self.nodes.contains_key(&rnuid);
                                        }
                                    },

                                    TestIntegerNegative => {
                                        ctrl.flags.success = ctrl.registers.integer < 0;
                                    },

                                    TestIntegerNonZero => {
                                        ctrl.flags.success = ctrl.registers.integer != 0;
                                    },

                                    TestIntegerPositive => {
                                        ctrl.flags.success = ctrl.registers.integer > 0;
                                    },

                                    TransmitInteger => {
                                        if ctrl.i_peer == 0 { // others are read-only
                                            let chan = ctrl.integer_channels[ctrl.i_integer_channel];
                                            if chan < self.ether_integers.len() {
                                                self.ether_integers[chan] = ctrl.registers.integer;
                                                ctrl.flags.success = true;    
                                            }
                                        }
                                    },

                                    TransmitOptuid => {
                                        self.ether_optuids[ctrl.optuid_channels[ctrl.i_optuid_channel]] = ctrl.optuids[ctrl.i_optuid];
                                        ctrl.flags.success = true;
                                    },

                                    Write => {
                                        if let Some(wnuid) = ctrl.data_optuids[ctrl.i_data_optuid] {
                                            self.nodes.entry(wnuid).and_modify(|wnode| {
                                                wnode.b_content = ctrl.registers.integer as u8;
                                                ctrl.data_optuids[ctrl.i_data_optuid] = Optuid::ot_bits(wnode.b_next);
                                                ctrl.flags.success = true;
                                            });
                                        }
                                    },

                                    ZeroInteger => {
                                        ctrl.registers.integer = 0;
                                        ctrl.flags.success = true;
                                    }
                                }

                                self.commands_count.entry(command)
                                    .and_modify(|n| { *n += 1 })
                                    .or_insert(1);
                            },

                            Content::Space | Content::Branch | Content::Construction(_) => {}
                        };

                        // update exec pointkey
                        if !next_exec_forced {
                            match econtent {
                                Content::Branch => {
                                    match ctrl.flags.success {
                                        false => {
                                            ctrl.exec_optuid = Optuid::ot_bits(enode.b_altnext);
                                        },
                                        true => {
                                            ctrl.exec_optuid = Optuid::ot_bits(enode.b_next);
                                        }
                                    };
                                    ctrl.flags.success = true;
                                },
                                _ => {
                                    ctrl.exec_optuid = Optuid::ot_bits(enode.b_next);
                                }
                            }
                        }

                        // increment ticks
                        ctrl.ticks += 1;

                        // Update original controller...
                        if self.controllers.contains_key(&cuid) { // ...if it has NOT been replaced by new controller created at this very execution
                            self.controllers.insert(cuid, ctrl);
                        }
                    } else {
                        self.remove_controller(& Some(cuid));
                    }
                } else {
                    self.remove_controller(& Some(cuid));
                }
            }
        }

        self.age += 1;

        // Background glitch
        if self.rng.gen_bool(self.glitch_background_prob) {
            if let Some((_, node)) = self.nodes.iter_mut().choose(&mut self.rng) {
                node.b_content = CONTENTS[self.rng.gen_range(0..CONTENTS.len())].to_bits();
                self.glitch_background_count += 1;
            }
        }        

        tick_data
    }

}