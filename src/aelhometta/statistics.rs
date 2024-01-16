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

use std::collections::HashMap;

use crate::serbin::OtBits;

use super::{
    Content,
    Ælhometta
};

pub struct ControllerGenerationStatistics {
    pub minimum: u128,
    pub average: u128,
    pub maximum: u128
}

pub struct ChannelsStatistics {
    pub optuids_some: usize,
    pub integers_nonzero: usize
}

impl Ælhometta {
    pub fn content_statistics(&self) -> HashMap<Content, usize> {
        let mut hm = HashMap::new();
        for (_, node) in & self.nodes {
            hm.entry(Content::ot_bits(node.b_content)).and_modify(|n| { *n += 1 }).or_insert(1);
        }
        hm
    }

    pub fn generation_statistics(&self) -> ControllerGenerationStatistics {
        let mut minimum: u128 = u128::MAX;
        let mut average: u128 = 0;
        let mut maximum: u128 = 0;
        let mut n: usize = 0;

        for (_, ctrl) in & self.controllers {
            let g = ctrl.generation;
            minimum = minimum.min(g);
            maximum = maximum.max(g);
            average += g;
            n += 1;
        }

        average /= (n as u128).max(1);

        ControllerGenerationStatistics {
            minimum,
            average,
            maximum
        }
    }

    pub fn channels_statistics(&self) -> ChannelsStatistics {
        let optuids_some: usize = self.ether_optuids.iter().map(|v| {
            v.is_some() as usize
        }).sum();
        let integers_nonzero: usize = self.ether_integers.iter().map(|&v| {
            (v != 0) as usize
        }).sum();
        ChannelsStatistics {
            optuids_some,
            integers_nonzero
        }
    }

}