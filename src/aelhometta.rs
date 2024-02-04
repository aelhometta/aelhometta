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

use enum_primitive_derive::Primitive;

use num_traits::{
    FromPrimitive,
    ToPrimitive
};

use rand::prelude::*;

use emyzelium::{
    DEF_PUBSUB_PORT,
    DEF_TOR_PROXY_HOST,
    DEF_TOR_PROXY_PORT,
    Efunguz
};

use std::fmt;

use std::{
    cmp::Ordering,
    collections::{
        HashMap,
        HashSet
    }
};

use crate::serbin::{
    OtBits,
    ToBits
};

mod ancestors;
mod iomap;
mod peer;
mod serbin;
mod statistics;
mod tick;

pub use serbin::FORMAT_VERSION;

// Constants; the lesser of them, the better (arbitrariness...)?

pub const NUM_CTRL_OPTUIDS: usize = 0x10;
pub const NUM_CTRL_DATA_OPTUIDS: usize = 8;
pub const NUM_CTRL_INTEGERS: usize = 0x20;

pub const NUM_OPTUID_CHANNELS: usize = 0x10000;
pub const NUM_INTEGER_CHANNELS: usize = 0x100000;

pub const NUM_CTRL_OPTUID_CHANNELS: usize = 0x10;
pub const NUM_CTRL_INTEGER_CHANNELS: usize = 0x20;

// Default values

const DEFAULT_MAX_NUM_CHAINS_BINLOG: u8 = 22; // 1 << this - maximum number of nodes and controllers

const DEFAULT_ÆLHOMETTA_FILENAME: &str = "aelhometta.bin";

pub type Uid = u32;
pub type Optuid = Option<Uid>;

pub type Integer = i64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Primitive, PartialOrd, Ord)]
pub enum Command {
    Abs                         = 1,
    Add                         = 2,
    BitAnd                      = 3,
    BitNot                      = 4,
    BitOr                       = 5,
    BitXor                      = 6,
    Construct                   = 7,
    DataOptuidIndexToInteger    = 8,
    Decrement                   = 9,
    Divide                      = 10,
    GetExecFromOptuid           = 11,
    GetIntegerFromIntegers      = 12,
    Increment                   = 13,
    Insert                      = 14,
    IntegerChannelToInteger     = 15,
    IntegerIndexToInteger       = 16,
    IntegerToDataOptuidIndex    = 17,
    IntegerToIntegerChannel     = 18,
    IntegerToIntegerIndex       = 19,
    IntegerToOptuidChannel      = 20,
    IntegerToOptuidIndex        = 21,
    IntegerToPeer               = 22,
    IntegerToSuccess            = 23,
    Multiply                    = 24,
    Negate                      = 25,
    NewChainAddInteger          = 26,
    NewChainAddIntegerChannel   = 27,
    NewChainAddOptuid           = 28,
    NewChainAddOptuidChannel    = 29,
    NewChainDetach              = 30,
    NewChainInitActive          = 31,
    NewChainInitPassive         = 32,
    NextDataOptuid              = 33,
    NextInteger                 = 34,
    NextIntegerChannel          = 35,
    NextOptuid                  = 36,
    NextOptuidChannel           = 37,
    NextPeer                    = 38,
    OptuidChannelToInteger      = 39,
    OptuidIndexToInteger        = 40,
    PeerToInteger               = 41,
    PreviousDataOptuid          = 42,
    PreviousInteger             = 43,
    PreviousIntegerChannel      = 44,
    PreviousOptuid              = 45,
    PreviousOptuidChannel       = 46,
    PreviousPeer                = 47,
    RandomContent               = 48,
    RandomInteger               = 49,
    Read                        = 50,
    ReceiveInteger              = 51,
    ReceiveOptuid               = 52,
    Remainder                   = 53,
    Remove                      = 54,
    Replicate                   = 55,
    Restart                     = 56,
    SetDataOptuidFromOptuid     = 57,
    SetIntegersFromInteger      = 58,
    SetOptuidFromDataOptuid     = 59,
    SetOptuidFromExec           = 60,
    ShiftDown                   = 61,
    ShiftUp                     = 62,
    Sign                        = 63,
    Skip                        = 64,
    Square                      = 65,
    Subtract                    = 66,
    SuccessToInteger            = 67,
    TestDataOptuid              = 68,
    TestIntegerNegative         = 69,
    TestIntegerNonZero          = 70,
    TestIntegerPositive         = 71,
    TransmitInteger             = 72,
    TransmitOptuid              = 73,
    Write                       = 74,
    ZeroInteger                 = 75
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Primitive, PartialOrd, Ord)]
pub enum Construction {
    AltNext         = 0,
    Discard         = 1,
    NextToStored    = 2,
    Restore         = 3,
    Store           = 4,
    Swap            = 5,
    Terminus        = 6
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)] // Debug is manual, to replace () with :
pub enum Content {
    Space,
    Branch,
    Command(Command),
    Construction(Construction)
}

pub const CONTENTS: [Content; 84] = [
    Content::Space,
    Content::Branch,
    Content::Command(Command::Abs),
    Content::Command(Command::Add),
    Content::Command(Command::BitAnd),
    Content::Command(Command::BitNot),
    Content::Command(Command::BitOr),
    Content::Command(Command::BitXor),
    Content::Command(Command::Construct),
    Content::Command(Command::DataOptuidIndexToInteger),
    Content::Command(Command::Decrement),
    Content::Command(Command::Divide),
    Content::Command(Command::GetExecFromOptuid),
    Content::Command(Command::GetIntegerFromIntegers),
    Content::Command(Command::Increment),
    Content::Command(Command::Insert),
    Content::Command(Command::IntegerChannelToInteger),
    Content::Command(Command::IntegerIndexToInteger),
    Content::Command(Command::IntegerToDataOptuidIndex),
    Content::Command(Command::IntegerToIntegerChannel),
    Content::Command(Command::IntegerToIntegerIndex),
    Content::Command(Command::IntegerToOptuidChannel),
    Content::Command(Command::IntegerToOptuidIndex),
    Content::Command(Command::IntegerToPeer),
    Content::Command(Command::IntegerToSuccess),
    Content::Command(Command::Multiply),
    Content::Command(Command::Negate),
    Content::Command(Command::NewChainAddInteger),
    Content::Command(Command::NewChainAddIntegerChannel),
    Content::Command(Command::NewChainAddOptuid),
    Content::Command(Command::NewChainAddOptuidChannel),
    Content::Command(Command::NewChainDetach),
    Content::Command(Command::NewChainInitActive),
    Content::Command(Command::NewChainInitPassive),
    Content::Command(Command::NextDataOptuid),
    Content::Command(Command::NextInteger),
    Content::Command(Command::NextIntegerChannel),
    Content::Command(Command::NextOptuid),
    Content::Command(Command::NextOptuidChannel),
    Content::Command(Command::NextPeer),
    Content::Command(Command::OptuidChannelToInteger),
    Content::Command(Command::OptuidIndexToInteger),
    Content::Command(Command::PeerToInteger),
    Content::Command(Command::PreviousDataOptuid),
    Content::Command(Command::PreviousInteger),
    Content::Command(Command::PreviousIntegerChannel),
    Content::Command(Command::PreviousOptuid),
    Content::Command(Command::PreviousOptuidChannel),
    Content::Command(Command::PreviousPeer),
    Content::Command(Command::RandomContent),
    Content::Command(Command::RandomInteger),
    Content::Command(Command::Read),
    Content::Command(Command::ReceiveInteger),
    Content::Command(Command::ReceiveOptuid),
    Content::Command(Command::Remainder),
    Content::Command(Command::Remove),
    Content::Command(Command::Replicate),
    Content::Command(Command::Restart),
    Content::Command(Command::SetDataOptuidFromOptuid),
    Content::Command(Command::SetIntegersFromInteger),
    Content::Command(Command::SetOptuidFromDataOptuid),
    Content::Command(Command::SetOptuidFromExec),
    Content::Command(Command::ShiftUp),
    Content::Command(Command::ShiftDown),
    Content::Command(Command::Sign),
    Content::Command(Command::Skip),
    Content::Command(Command::Square),
    Content::Command(Command::Subtract),
    Content::Command(Command::SuccessToInteger),
    Content::Command(Command::TestDataOptuid),
    Content::Command(Command::TestIntegerNegative),
    Content::Command(Command::TestIntegerNonZero),
    Content::Command(Command::TestIntegerPositive),
    Content::Command(Command::TransmitInteger),
    Content::Command(Command::TransmitOptuid),
    Content::Command(Command::Write),
    Content::Command(Command::ZeroInteger),
    Content::Construction(Construction::AltNext),
    Content::Construction(Construction::Discard),
    Content::Construction(Construction::NextToStored),
    Content::Construction(Construction::Restore),
    Content::Construction(Construction::Store),
    Content::Construction(Construction::Swap),
    Content::Construction(Construction::Terminus)
];

#[derive(Clone, Copy)]
pub struct Node {
    b_content: u8,
    b_next: Uid,
    b_altnext: Uid
}

#[derive(Clone, Copy)]
pub struct Registers {
    integer: Integer
}

#[derive(Clone, Copy)]
pub struct Flags {
    success: bool
}

#[derive(Clone)]
pub struct Controller {
    chain_start_optuid: Optuid,
    
    exec_optuid: Optuid,

    data_optuids: Vec<Optuid>, // for read/write loops: each, when selected, advances to next at data access (Read, Write, etc.)
    i_data_optuid: usize,

    new_chain_optuid: Optuid,
    new_controller: Option<Box<Self>>,

    registers: Registers,
    flags: Flags,

    optuids: Vec<Optuid>,
    i_optuid: usize,

    integers: Vec<Integer>,
    i_integer: usize,

    optuid_channels: Vec<usize>,
    i_optuid_channel: usize,

    i_peer: usize, // 0th is "this". Used by ReceiveInteger (all peers), TransmitInteger (only this peer), etc.

    integer_channels: Vec<usize>, // may be used with other peers (i_peer > 0), so values can be greater than size of this peer's integer ether
    i_integer_channel: usize,

    generation: u128,
    ticks: u128
}

#[derive(Clone)]
pub struct OtherPeer {
    publickey: String,
    onion: String,
    port: u16,
    ether_integers: Vec<Integer>,
    ut_last_update: i64 // microseconds since Unix epoch
}

#[derive(Clone)]
pub struct IntegersFileMapping {
    start: usize, // index of integer channel
    length: usize, // number of integer channels from the start
    interval: i64, // microseconds
    filepath: String, // must be without spaces
    ut_last_update: i64, // microseconds since Unix epoch
}

pub struct Ælhometta {
    // Serialisable part
    max_num_chains_binlog: u8,

    new_node_uid: Uid,
    nodes: HashMap<Uid, Node>,
    nodes_historing: Vec<Optuid>,
    i_nodes_historing: usize,

    new_controller_uid: Uid,
    controllers: HashMap<Uid, Controller>,
    controllers_historing: Vec<Optuid>,
    i_controllers_historing: usize,

    commandswitch: u128, // i-th bit is 0 means i-th command is NOPped. Will there always be less than 128 commands?..

    ether_optuids: Vec<Optuid>,
    ether_integers: Vec<Integer>,

    age: u128, // ticks performed

    spaces_count: u128,
    branches_main_count: u128,
    branches_alt_count: u128,
    commands_count: HashMap<Command, u128>,
    constructions_count: HashMap<Construction, u128>, // "special": updated from within execution of Construct command

    glitch_background_prob: f64, // probability per tick of random node changing its content randomly
    glitch_background_count: u128,
    glitch_replicate_prob: f64, // probability per node of replicated node changing its content randomly
    glitch_replicate_count: u128,
    glitch_construct_prob: f64, // probability per node of node read at construction changing its content randomly
    glitch_construct_count: u128,

    // Peer-related
    share_size: usize, // number of integer channels to emit
    share_interval: i64, // microseconds between emits; 0 means "never"

    ut_last_share: i64, // microseconds since Unix epoch

    secretkey: String,
    port: u16,
    torproxy_port: u16,
    torproxy_host: String,

    exposed: bool,

    other_peers: Vec<OtherPeer>,

    whitelist: HashSet<String>,

    in_permitted_before_num: u64,
    in_attempted_before_num: u64,

    // IO-related
    output_mappings: Vec<IntegersFileMapping>,
    input_mappings: Vec<IntegersFileMapping>,

    // Non-serialisable part
    max_num_chains: usize,
    max_num_chains_binmask: usize,

    rng: ThreadRng,

    efunguz: Option<Efunguz>,
}

pub struct TickData {
    pub controller_optuid: Optuid,
    pub exec_optuid: Optuid,
    pub exec_optcontent: Option<Content>
}

pub trait Hexly {
    fn hexly(&self) -> String;
}

fn new_commands_count() -> HashMap<Command, u128> {
    let mut commands_count = HashMap::new();
    for content in CONTENTS {
        match content {
            Content::Command(command) => {
                commands_count.insert(command, 0);
            },
            _ => {}
        }
    }
    commands_count
}

pub fn new_constructions_count() -> HashMap<Construction, u128> {
    let mut constructions_count = HashMap::new();
    for content in CONTENTS {
        match content {
            Content::Construction(construction) => {
                constructions_count.insert(construction, 0);
            },
            _ => {}
        }
    }
    constructions_count
}

impl ToBits<u8> for Content {
    fn to_bits(&self) -> u8 {
        let (high, low) = match *self {
            Content::Space => (0, 0),
            Content::Branch => (0, 0x7F),
            Content::Command(comm) => (0, comm.to_u8().unwrap_or(0)),
            Content::Construction(constr) => (1, constr.to_u8().unwrap_or(0))
        };
        (high << 7) | low
    }
}

impl OtBits<u8> for Content {
    fn ot_bits(x: u8) -> Self {
        let (high, low) = (x >> 7, x & 0x7F);
        match high {
            0 => match low {
                0 => Self::Space,
                0x7F => Self::Branch,
                _ => match Command::from_u8(low) {
                    Some(comm) => Self::Command(comm),
                    None => Self::Space
                }
            },
            1 => {
                match Construction::from_u8(low) {
                    Some(constr) => Self::Construction(constr),
                    None => Self::Space
                }
            },
            _ => Self::Space
        }
    }
}

impl ToBits<Uid> for Optuid {
    fn to_bits(&self) -> Uid {
        match *self {
            Some(uid) => 0x80000000 | (uid & 0x7FFFFFFF),
            None => 0
        }
    }
}

impl OtBits<Uid> for Optuid {
    fn ot_bits(x: Uid) -> Self {
        if x & 0x80000000 != 0 {
            Some(x & 0x7FFFFFFF)
        } else {
            None
        }
    }
}

impl Node {
    fn new(b_content: u8, b_next: Uid, b_altnext: Uid) -> Self {
        Self {
            b_content,
            b_next,
            b_altnext
        }
    }

    pub fn b_content(&self) -> u8 {
        self.b_content
    }

    pub fn b_next(&self) -> Uid {
        self.b_next
    }

    pub fn b_altnext(&self) -> Uid {
        self.b_altnext
    }

}

impl Registers {
    fn new() -> Self {
        Self {
            integer: 0
        }
    }

    pub fn integer(&self) -> Integer {
        self.integer
    }

}

impl Flags {
    fn new() -> Self {
        Self {
            success: true
        }
    }

    pub fn success(&self) -> bool {
        self.success
    }
}

impl Controller {
    fn new() -> Self {
        Self {
            chain_start_optuid: None,
            exec_optuid: None,
            data_optuids: vec![None; NUM_CTRL_DATA_OPTUIDS],
            i_data_optuid: 0,
            new_chain_optuid: None,
            new_controller: None,
            registers: Registers::new(),
            flags: Flags::new(),
            optuids: vec![None; NUM_CTRL_OPTUIDS],
            i_optuid: 0,
            integers: vec![0; NUM_CTRL_INTEGERS],
            i_integer: 0,
            optuid_channels: vec![0; NUM_CTRL_OPTUID_CHANNELS],
            i_optuid_channel: 0,
            i_peer: 0,
            integer_channels: vec![0; NUM_CTRL_INTEGER_CHANNELS],
            i_integer_channel: 0,
            generation: 0,
            ticks: 0
        }
    }

    pub fn chain_start_optuid(&self) -> Optuid {
        self.chain_start_optuid
    }

    pub fn exec_optuid(&self) -> Optuid {
        self.exec_optuid
    }

    pub fn data_optuids(&self) -> & Vec<Optuid> {
        & self.data_optuids
    }

    pub fn i_data_optuid(&self) -> usize {
        self.i_data_optuid
    }

    pub fn new_chain_optuid(&self) -> Optuid {
        self.new_chain_optuid
    }

    pub fn new_controller_is_some(&self) -> bool {
        self.new_controller.is_some()
    }

    pub fn registers(&self) -> Registers {
        self.registers
    }

    pub fn flags(&self) -> Flags {
        self.flags
    }

    pub fn optuids(&self) -> & Vec<Optuid> {
        & self.optuids
    }

    pub fn i_optuid(&self) -> usize {
        self.i_optuid
    }
    
    pub fn integers(&self) -> & Vec<Integer> {
        & self.integers
    }

    pub fn i_integer(&self) -> usize {
        self.i_integer
    }

    pub fn optuid_channels(&self) -> & Vec<usize> {
        & self.optuid_channels
    }

    pub fn i_optuid_channel(&self) -> usize {
        self.i_optuid_channel
    }

    pub fn integer_channels(&self) -> & Vec<usize> {
        & self.integer_channels
    }

    pub fn i_integer_channel(&self) -> usize {
        self.i_integer_channel
    }

    pub fn generation(&self) -> u128 {
        self.generation
    }

    pub fn ticks(&self) -> u128 {
        self.ticks
    }

}

impl OtherPeer {
    pub fn new(publickey: &str, onion: &str, port: u16) -> Self {
        Self {
            publickey: publickey.to_string(),
            onion: onion.to_string(),
            port,
            ether_integers: Vec::new(),
            ut_last_update: -1 
        }
    }

    pub fn publickey(&self) -> String {
        self.publickey.clone()
    }

    pub fn onion(&self) -> String {
        self.onion.clone()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn ether_integers(&self) -> & Vec<Integer> {
        & self.ether_integers
    }

    pub fn ut_last_update(&self) -> i64 {
        self.ut_last_update
    }

}

impl IntegersFileMapping {
    pub fn new(start: usize, length: usize, interval: i64, filepath: &str) -> Self {
        Self {
            start,
            length,
            interval,
            filepath: filepath.to_string(),
            ut_last_update: -1 
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn interval(&self) -> i64 {
        self.interval
    }

    pub fn filepath(&self) -> String {
        self.filepath.clone()
    }

    pub fn ut_last_update(&self) -> i64 {
        self.ut_last_update
    }

}

impl Ælhometta {
    fn add_new_node(&mut self, content: Content) -> Optuid {
        let nuid = self.new_node_uid;
        self.new_node_uid = (self.new_node_uid + 1) & 0x7FFFFFFF;
        let node = Node::new(content.to_bits(), None.to_bits(), None.to_bits());
        if let Some(onuid) = self.nodes_historing[self.i_nodes_historing] {
            self.nodes.remove(&onuid);
        }
        self.nodes.insert(nuid, node);
        self.nodes_historing[self.i_nodes_historing] = Some(nuid);
        self.i_nodes_historing = (self.i_nodes_historing + 1) & self.max_num_chains_binmask;
        Some(nuid)    
    }

    fn add_new_node_to_existing(&mut self, content: Content, toouid: &Optuid) -> Optuid {
        if let &Some(touid) = toouid {
            let ouid = self.add_new_node(content);
            let mut aouid: Optuid = None;
            self.nodes.entry(touid).and_modify(|tonode| {
                tonode.b_next = ouid.to_bits();
                aouid = ouid;
            });
            aouid
        } else { None }
    }

    fn remove_node(&mut self, rouid: &Optuid) -> Optuid {
        if let &Some(ruid) = rouid {
            if self.nodes.contains_key(&ruid) {
                let b_ruid = 0x80000000 | ruid;
                let b_r_next = self.nodes.get(&ruid).unwrap().b_next;
                self.nodes.remove(&ruid);
                for ouid in &mut self.nodes_historing { // slow...
                    if let &mut Some(uid) = ouid {
                        if uid == ruid {
                            *ouid = None;
                            break;
                        }
                    }
                }
                for (_, node) in &mut self.nodes { // slow...
                    if node.b_next == b_ruid {
                        node.b_next = b_r_next;
                    }
                    if node.b_altnext == b_ruid {
                        node.b_next = b_r_next;
                    }
                }
                Some(ruid)
            } else { None }
        } else { None }
    }

    fn add_controller(&mut self, ctrl: Controller) -> Optuid {
        let cuid = self.new_controller_uid;
        self.new_controller_uid = (self.new_controller_uid + 1) & 0x7FFFFFFF;
        if let Some(ocuid) = self.controllers_historing[self.i_controllers_historing] {
            self.controllers.remove(&ocuid);
        }
        self.controllers.insert(cuid, ctrl);
        self.controllers_historing[self.i_controllers_historing] = Some(cuid);
        self.i_controllers_historing = (self.i_controllers_historing + 1) & self.max_num_chains_binmask;
        Some(cuid)
    }

    fn remove_controller(&mut self, rouid: &Optuid) -> Optuid {
        if let &Some(ruid) = rouid {
            if self.controllers.contains_key(&ruid) {
                self.controllers.remove(&ruid);
                for ouid in &mut self.controllers_historing { // slow...
                    if let Some(uid) = ouid {
                        if *uid == ruid {
                            *ouid = None;
                            break;
                        }
                    }
                }
                Some(ruid)
            } else { None }
        } else { None }
    }

    fn add_linear_passive_chain(&mut self, contents: &Vec<Content>) -> Optuid {
        let snouid = self.add_new_node(Content::Space);
        let mut nouid = snouid;
        for content in contents {
            if let Some(_) = nouid {
                nouid = self.add_new_node_to_existing(*content, &nouid);
            } else {
                break;
            }
        }
        snouid
    }

    pub fn new(max_num_chains_binlog: u8) -> Self {
        let rng = thread_rng();
        let max_num_chains: usize = 1 << max_num_chains_binlog;

        let mut commandswitch = u128::MAX;
        commandswitch ^= 1 << Command::GetExecFromOptuid.to_u8().unwrap_or(0);
        commandswitch ^= 1 << Command::SetOptuidFromExec.to_u8().unwrap_or(0);

        Self {
            max_num_chains_binlog,
            max_num_chains,
            max_num_chains_binmask: max_num_chains - 1,
            new_node_uid: 0,
            nodes: HashMap::new(),
            nodes_historing: vec![None; max_num_chains],
            i_nodes_historing: 0,
            new_controller_uid: 0,
            controllers: HashMap::new(),
            controllers_historing: vec![None; max_num_chains],
            i_controllers_historing: 0,
            commandswitch,
            ether_optuids: vec![None; NUM_OPTUID_CHANNELS],
            ether_integers: vec![0; NUM_INTEGER_CHANNELS],
            age: 0,
            spaces_count: 0,
            branches_main_count: 0,
            branches_alt_count: 0,
            commands_count: new_commands_count(),
            constructions_count: new_constructions_count(),
            glitch_background_prob: 0.0,
            glitch_background_count: 0,
            glitch_replicate_prob: 0.0,
            glitch_replicate_count: 0,
            glitch_construct_prob: 0.0,
            glitch_construct_count: 0,
            share_size: 0,
            share_interval: 0,
            ut_last_share: -1,
            secretkey: String::new(),
            port: DEF_PUBSUB_PORT,
            torproxy_port: DEF_TOR_PROXY_PORT,
            torproxy_host: String::from(DEF_TOR_PROXY_HOST),
            exposed: false,
            other_peers: Vec::new(),
            whitelist: HashSet::new(),
            in_permitted_before_num: 0,
            in_attempted_before_num: 0,
            output_mappings: Vec::new(),
            input_mappings: Vec::new(),

            rng,
            efunguz: None,
        }
    }

    pub fn new_default() -> Self {
        Self::new(
            DEFAULT_MAX_NUM_CHAINS_BINLOG
        )
    }

    pub fn age(&self) -> u128 {
        self.age
    }

    pub fn spaces_count(&self) -> u128 {
        self.spaces_count
    }

    pub fn branches_main_count(&self) -> u128 {
        self.branches_main_count
    }

    pub fn branches_alt_count(&self) -> u128 {
        self.branches_alt_count
    }

    pub fn commands_count(&self) -> & HashMap<Command, u128> {
        & self.commands_count
    }

    pub fn constructions_count(&self) -> & HashMap<Construction, u128> {
        & self.constructions_count
    }

    pub fn max_num_chains_binlog(&self) -> u8 {
        self.max_num_chains_binlog
    }

    pub fn max_num_chains(&self) -> usize {
        self.max_num_chains
    }

    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn num_controllers(&self) -> usize {
        self.controllers.len()
    }

    pub fn mem_usage(&self) -> usize {
        use std::mem::size_of;
        self.nodes.capacity() * (size_of::<Node>() + size_of::<Uid>())
        + self.nodes_historing.capacity() * size_of::<Optuid>()
        + self.controllers.capacity() * (size_of::<Controller>() + size_of::<Uid>())
        + self.controllers_historing.capacity() * size_of::<Optuid>()
        + NUM_OPTUID_CHANNELS * size_of::<Optuid>()
        + NUM_INTEGER_CHANNELS * size_of::<Integer>()

        + self.controllers.iter().map(|(_, ctrl)| {
            if ctrl.new_controller.is_some() { size_of::<Controller>() } else { 0 }
        }).sum::<usize>()

        + self.other_peers.iter().map(|peer|
            peer.ether_integers.len() * size_of::<Integer>()
        ).sum::<usize>()

        + (self.output_mappings.capacity() + self.input_mappings.capacity()) * size_of::<IntegersFileMapping>()
    }

    pub fn random_node_optuid(&mut self) -> Optuid {
        self.nodes.iter().choose(&mut self.rng).map(|(nuid, _)| *nuid)
    }

    pub fn random_node_with_bcontent_optuid(&mut self, b_content: u8) -> Optuid {
        let mut n: usize = 0;
        for (_, node) in & self.nodes {
            if node.b_content == b_content {
                n += 1;
            }
        }
        if n > 0 {
            let i = self.rng.gen_range(0..n);
            let mut j: usize = 0;
            for (uid, node) in & self.nodes {
                if node.b_content == b_content {
                    if j == i {
                        return Some(*uid);
                    }
                    j += 1;
                }
            }
            None // should be unreachable
        } else {
            None
        }
    }

    pub fn node(&self, nuid: &Uid) -> Option<&Node> {
        self.nodes.get(nuid)
    }

    pub fn previous_nodes(&self, nuid: &Uid) -> Vec<(Uid, bool, Content)> {
        let mut prev_nodes = Vec::<(Uid, bool, Content)>::new();
        for (uid, node) in & self.nodes {
            if let Some(nextuid) = Optuid::ot_bits(node.b_next) {
                if *nuid == nextuid {
                    prev_nodes.push((*uid, true, Content::ot_bits(node.b_content)));
                }
            }
            if let Some(nextuid) = Optuid::ot_bits(node.b_altnext) {
                if *nuid == nextuid {
                    prev_nodes.push((*uid, false, Content::ot_bits(node.b_content)));
                }
            }
        }
        prev_nodes
    }

    pub fn random_controller_optuid(&mut self) -> Optuid {
        self.controllers.iter().choose(&mut self.rng).map(|(cuid, _)| *cuid)
    }

    pub fn controller(&self, cuid: &Uid) -> Option<&Controller> {
        self.controllers.get(cuid)
    }

    pub fn commandswitch(&self, i: u8) -> bool {
        (self.commandswitch >> i) & 1 != 0
    }

    pub fn change_commandswitch(&mut self, i: u8) {
        self.commandswitch ^= 1 << i
    }

    pub fn ether_optuids(&self) -> & Vec<Optuid> {
        & self.ether_optuids
    }

    pub fn ether_integers(&self) -> & Vec<Integer> {
        & self.ether_integers
    }

    pub fn glitch_background_prob(&self) -> f64 {
        self.glitch_background_prob
    }

    pub fn set_glitch_background_prob(&mut self, p: f64) {
        self.glitch_background_prob = p.max(0.0).min(1.0);
    }

    pub fn glitch_background_count(&self) -> u128 {
        self.glitch_background_count
    }

    pub fn glitch_replicate_prob(&self) -> f64 {
        self.glitch_replicate_prob
    }

    pub fn set_glitch_replicate_prob(&mut self, p: f64) {
        self.glitch_replicate_prob = p.max(0.0).min(1.0);
    }

    pub fn glitch_replicate_count(&self) -> u128 {
        self.glitch_replicate_count
    }

    pub fn glitch_construct_prob(&self) -> f64 {
        self.glitch_construct_prob
    }

    pub fn set_glitch_construct_prob(&mut self, p: f64) {
        self.glitch_construct_prob = p.max(0.0).min(1.0);
    }

    pub fn glitch_construct_count(&self) -> u128 {
        self.glitch_construct_count
    }

    pub fn share_size(&self) -> usize {
        self.share_size
    }
    
    pub fn share_interval(&self) -> i64 {
        self.share_interval
    }

    pub fn ut_last_share(&self) -> i64 {
        self.ut_last_share
    }

    pub fn secretkey(&self) -> String { // where's security?..
        self.secretkey.clone()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn torproxy_port(&self) -> u16 {
        self.torproxy_port
    }

    pub fn torproxy_host(&self) -> String {
        self.torproxy_host.clone()
    }

    pub fn exposed(&self) -> bool {
        self.exposed
    }

    pub fn in_absorbing_num(&self) -> Option<u64> {
        self.efunguz.as_ref().map(|efunguz| efunguz.in_absorbing_num())
    }

    pub fn in_permitted_num(&self) -> u64 {
        self.in_permitted_before_num + match self.efunguz {
            Some(ref efunguz) => efunguz.in_permitted_num(),
            None => 0
        }
    }

    pub fn in_attempted_num(&self) -> u64 {
        self.in_attempted_before_num + match self.efunguz {
            Some(ref efunguz) => efunguz.in_attempted_num(),
            None => 0
        }
    }

    pub fn other_peers(&self) -> & Vec<OtherPeer> {
        & self.other_peers
    }

    pub fn output_mappings(&self) -> & Vec<IntegersFileMapping> {
        & self.output_mappings
    }

    pub fn input_mappings(&self) -> & Vec<IntegersFileMapping> {
        & self.input_mappings
    }

    pub fn cleanse(&mut self) {
        self.new_node_uid = 0;
        self.nodes.clear();
        self.nodes_historing = vec![None; self.max_num_chains];
        self.i_nodes_historing = 0;

        self.new_controller_uid = 0;
        self.controllers.clear();
        self.controllers_historing = vec![None; self.max_num_chains];
        self.i_controllers_historing = 0;

        self.commandswitch = u128::MAX;
        self.commandswitch ^= 1 << Command::GetExecFromOptuid.to_u8().unwrap_or(0);
        self.commandswitch ^= 1 << Command::SetOptuidFromExec.to_u8().unwrap_or(0);

        self.ether_optuids = vec![None; NUM_OPTUID_CHANNELS];
        self.ether_integers = vec![0; NUM_INTEGER_CHANNELS];

        self.age = 0;
        
        self.spaces_count = 0;
        self.branches_main_count = 0;
        self.branches_alt_count = 0;
        self.commands_count = new_commands_count();
        self.constructions_count = new_constructions_count();

        self.glitch_background_prob = 0.0;
        self.glitch_background_count = 0;

        self.glitch_replicate_prob = 0.0;
        self.glitch_replicate_count = 0;

        self.glitch_construct_prob = 0.0;
        self.glitch_construct_count = 0;

        self.share_size = 0;
        self.share_interval = 0;

        self.ut_last_share = -1;

        self.secretkey = String::new();
        self.port = DEF_PUBSUB_PORT;
        self.torproxy_port = DEF_TOR_PROXY_PORT;
        self.torproxy_host = DEF_TOR_PROXY_HOST.to_string();

        self.exposed = false;

        self.other_peers.clear();

        self.whitelist.clear();
        
        self.in_permitted_before_num = 0;
        self.in_attempted_before_num = 0;

        self.output_mappings.clear();
        self.input_mappings.clear();

        self.rng = thread_rng();

        self.efunguz = None;
    }

    pub fn change_limit(&mut self, max_num_chains_binlog: u8) {
        let max_num_chains: usize = 1 << max_num_chains_binlog;
        match self.max_num_chains.cmp(&max_num_chains) {
            Ordering::Less => { // insert
                let n = max_num_chains - self.max_num_chains;

                // Nodes
                let mut nodes_historing = self.nodes_historing[0..self.i_nodes_historing].to_vec();
                nodes_historing.extend(vec![None; n]);
                nodes_historing.extend(self.nodes_historing[self.i_nodes_historing..self.max_num_chains].to_vec());
                self.nodes_historing = nodes_historing;

                // Controllers
                let mut ctrls_historing = self.controllers_historing[0..self.i_controllers_historing].to_vec();
                ctrls_historing.extend(vec![None; n]);
                ctrls_historing.extend(self.controllers_historing[self.i_controllers_historing..self.max_num_chains].to_vec());
                self.controllers_historing = ctrls_historing;

                self.max_num_chains_binlog = max_num_chains_binlog;
                self.max_num_chains = max_num_chains;
                self.max_num_chains_binmask = max_num_chains - 1;
            },
            Ordering::Greater => { // remove oldest
                let n = self.max_num_chains - max_num_chains;
                
                // Nodes
                self.nodes_historing.rotate_left(self.i_nodes_historing);
                for i in 0..n {
                    if let Some(nuid) = self.nodes_historing[i] {
                        self.nodes.remove(&nuid);
                    }                    
                }
                self.nodes.shrink_to(max_num_chains);
                self.nodes_historing = self.nodes_historing[n..self.max_num_chains].to_vec();
                self.i_nodes_historing = 0;

                // Controllers
                self.controllers_historing.rotate_left(self.i_controllers_historing);
                for i in 0..n {
                    if let Some(cuid) = self.controllers_historing[i] {
                        self.controllers.remove(&cuid);
                    }                    
                }
                self.controllers.shrink_to(max_num_chains);
                self.controllers_historing = self.controllers_historing[n..self.max_num_chains].to_vec();
                self.i_controllers_historing = 0;

                self.max_num_chains_binlog = max_num_chains_binlog;
                self.max_num_chains = max_num_chains;
                self.max_num_chains_binmask = max_num_chains - 1;
            },
            _ => {} // nothing to do if equal
        }
    }

}

impl TickData {
    pub fn new_default() -> Self {
        Self {
            controller_optuid: None,
            exec_optuid: None,
            exec_optcontent: None
        }
    }

}

impl Hexly for Optuid {
    fn hexly(&self) -> String {
        match *self {
            Some(uid) => format!("{:08X}", uid),
            None => String::from("×")
        }
    }
}

impl fmt::Debug for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Content::Space => {
                write!(f, "Space")
            },
            Content::Branch => {
                write!(f, "Branch")
            },
            Content::Command(command) => {
                write!(f, "Command:{:?}", command)
            },
            Content::Construction(construction) => {
                write!(f, "Construction:{:?}", construction)
            }
        }
    }
}

