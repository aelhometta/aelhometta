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

use super::{
    Ælhometta,
    Controller,
    Content,
    Command,
    Construction
};

impl Ælhometta {
    pub fn introduce_ancestor_a(&mut self, sterile_power: u8, skip_power: u8, spacity: usize) {
        use Content::*;
        use self::Command::*;
        use self::Construction::*;

        // Scheme of 
        // 1) constructor that copies scheme, builds constructor and jumbler,
        // 2) jumbler that randomly modifies scheme
        let mut scheme = vec![
            // Constructor scheme
            Command(ZeroInteger)
        ];

        // "Compile" sterile_power into scheme
        let l2 = 8 - sterile_power.leading_zeros();
        let mut sterile_power_reversed = sterile_power.reverse_bits();
        sterile_power_reversed >>= sterile_power_reversed.trailing_zeros();
        for _ in 0..l2 {
            scheme.push(Command(ShiftUp));
            if sterile_power_reversed & 1 != 0 {
                scheme.push(Command(Increment));
            }
            sterile_power_reversed >>= 1;
        }
        // registers.integer = sterile_power

        scheme.extend(vec![
            Command(SetIntegersFromInteger),
            Command(NextInteger),

            Command(PreviousInteger),
            Construction(Store),
            Command(GetIntegerFromIntegers),
            Command(NextInteger),
            Command(SetIntegersFromInteger),        // ints[1] = ints[0]

            Command(NextInteger),
            Command(ZeroInteger),
            Command(Increment),
            Command(SetIntegersFromInteger),
            Command(PreviousInteger),
            
            Command(GetIntegerFromIntegers),
            Construction(Store),
            Command(TestIntegerNonZero),
            Branch,
            Construction(Store),
            Command(Decrement),
            Command(SetIntegersFromInteger),
            Command(NextInteger),
            Command(RandomInteger),
            Command(BitAnd),
            Command(SetIntegersFromInteger),
            Command(PreviousInteger),
            Construction(Swap),
            Construction(NextToStored),
            Construction(Discard),
            Construction(Restore),
            Construction(Discard),
            Construction(AltNext),
            Command(NextInteger),
            Command(GetIntegerFromIntegers), // P{registers.integer = 1} = 1 - P{Sterile} = 2^(-S)
            Command(PreviousInteger),

            Command(IntegerToSuccess),
            Branch,
            Construction(AltNext),
            Construction(NextToStored),            

            // Copy scheme
            Command(SetDataOptuidFromOptuid),         // read -> self scheme
            Command(NextOptuid),
            Command(NewChainInitPassive),
            Command(PreviousOptuid),
            Command(Skip),                  // skip Space at scheme start
            Command(Replicate),
            Command(NewChainDetach),

            // Build constructor from scheme
            Command(SetDataOptuidFromOptuid),         // read -> self scheme
            Command(NextOptuid),
            Command(NextOptuid),
            Command(NewChainInitActive),
            Command(Skip),              // skip Space at scheme start
            Command(Construct),         // until Terminus
            Command(PreviousOptuid),
            Command(NewChainAddOptuid),
            Command(NewChainDetach),

            // Build jumbler from scheme
            Command(NextOptuid),
            Command(NewChainInitActive),
            Command(Construct),         // until end of scheme
            Command(PreviousOptuid),
            Command(NewChainAddOptuid),
            Command(NewChainDetach),

            Command(PreviousOptuid),

            Construction(NextToStored),
            Construction(Discard),          // probably unnecessary

            Construction(Terminus),

            // Jumbler scheme
            //
            // integers[0] to be M for 2^(-M) prob. of mutation per node,
            // ints[1] to be loop counter (starting from ints[0]),
            // ints[2] to be or-accumulator (don't forget to reset)

            Command(ZeroInteger)
        ]);

        // "Compile" skip_power into scheme
        let l2 = 8 - skip_power.leading_zeros();
        let mut skip_power_reversed = skip_power.reverse_bits();
        skip_power_reversed >>= skip_power_reversed.trailing_zeros();
        for _ in 0..l2 {
            scheme.push(Command(ShiftUp));
            if skip_power_reversed & 1 != 0 {
                scheme.push(Command(Increment));
            }
            skip_power_reversed >>= 1;
        }
        // registers.integer = skip_power

        scheme.extend(vec![
            Command(SetIntegersFromInteger),
            Command(NextInteger),

            Command(SetDataOptuidFromOptuid),
            Construction(Store),
            Command(Skip),              // skip Space at scheme start
            Command(TestDataOptuid),
            Construction(Store),
            Construction(Swap),
            Branch,
            Construction(AltNext),
            Construction(NextToStored),
            Construction(Discard),
            Command(RandomContent),

            Command(PreviousInteger),
            Command(GetIntegerFromIntegers),
            Command(NextInteger),
            Command(SetIntegersFromInteger),        // ints[1] = ints[0]

            Command(NextInteger),
            Command(ZeroInteger),
            Command(SetIntegersFromInteger),
            Command(PreviousInteger),
            
            Command(GetIntegerFromIntegers),
            Construction(Store),
            Command(TestIntegerNonZero),
            Branch,
            Construction(Store),
            Command(Decrement),
            Command(SetIntegersFromInteger),
            Command(NextInteger),
            Command(RandomInteger),
            Command(BitOr),
            Command(SetIntegersFromInteger),
            Command(PreviousInteger),
            Construction(Swap),
            Construction(NextToStored),
            Construction(Discard),
            Construction(Restore),
            Construction(Discard),
            Construction(AltNext),
            Command(NextInteger),
            Command(GetIntegerFromIntegers), // P{registers.integer = 1} = P{Skip} = 1 - 2^(-M)
            Command(PreviousInteger),

            Command(IntegerToSuccess),
            Branch,
            Construction(Store),
            Command(Skip),
            Construction(Swap),
            Construction(NextToStored),
            Construction(Swap),
            Construction(Restore),
            Construction(Discard),
            Construction(AltNext),
            Command(RandomInteger),
            Command(IntegerToSuccess),
            Branch,
            Construction(Store),
            Command(Write),
            Construction(Swap),
            Construction(NextToStored),
            Construction(Swap),
            Construction(Restore),
            Construction(Discard),
            Construction(AltNext),
            Command(RandomInteger),
            Command(IntegerToSuccess),
            Branch,
            Construction(Store),
            Command(Insert),
            Construction(Swap),
            Construction(NextToStored),
            Construction(Swap),
            Construction(Restore),
            Construction(Discard),
            Construction(AltNext),
            Command(Remove),

            Construction(NextToStored),
            Construction(Discard)
        ]);

        // "Spacify": insert spaces before all but Construction-s
        let mut scheme_spaced = vec![];
        let spaces = vec![Space; spacity];
        for content in scheme {
            if let Construction(..) = content {
            } else {
                scheme_spaced.extend(spaces.iter());
            }
            scheme_spaced.push(content);
        }

        let scheme_ouid = self.add_linear_passive_chain(&scheme_spaced);

        // Constructor, accompanying the scheme
        let constr_chain_start_ouid = self.add_new_node(Space);

        let mut nouid = constr_chain_start_ouid;

        // Copying of scheme
        nouid = self.add_new_node_to_existing(Command(SetDataOptuidFromOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NextOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NewChainInitPassive), &nouid);
        nouid = self.add_new_node_to_existing(Command(PreviousOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(Skip), &nouid);
        nouid = self.add_new_node_to_existing(Command(Replicate), &nouid);
        nouid = self.add_new_node_to_existing(Command(NewChainDetach), &nouid);

        // Constructor
        nouid = self.add_new_node_to_existing(Command(SetDataOptuidFromOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NextOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NextOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NewChainInitActive), &nouid);
        nouid = self.add_new_node_to_existing(Command(Skip), &nouid);
        nouid = self.add_new_node_to_existing(Command(Construct), &nouid);
        nouid = self.add_new_node_to_existing(Command(PreviousOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NewChainAddOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NewChainDetach), &nouid);

        // Jumbler
        nouid = self.add_new_node_to_existing(Command(NextOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NewChainInitActive), &nouid);
        nouid = self.add_new_node_to_existing(Command(Construct), &nouid);
        nouid = self.add_new_node_to_existing(Command(PreviousOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NewChainAddOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NewChainDetach), &nouid);

        self.add_new_node_to_existing(Command(PreviousOptuid), &nouid);
        // Not looped, runs only once

        let mut constr_ctrl = Controller::new();
        constr_ctrl.chain_start_optuid = constr_chain_start_ouid;
        constr_ctrl.optuids[0] = scheme_ouid;
        constr_ctrl.exec_optuid = constr_ctrl.chain_start_optuid;

        self.add_controller(constr_ctrl);

        // Jumbler, accompanying the scheme...
        // ...will be built by constructor of next (1st) generation
    }

    pub fn introduce_ancestor_b(&mut self, spacity: usize) {
        use Content::*;
        use self::Command::*;
        use self::Construction::*;

        // Scheme of 
        // 1) constructor that copies scheme and builds constructor
        let scheme = vec![
            Construction(Store),
            // Constructor scheme

            // Copy scheme
            Command(SetDataOptuidFromOptuid),         // read -> self scheme
            Command(NextOptuid),
            Command(NewChainInitPassive),
            Command(PreviousOptuid),
            Command(Skip),                  // skip Space at scheme start
            Command(Replicate),
            Command(NewChainDetach),

            // Build constructor from scheme
            Command(SetDataOptuidFromOptuid),         // read -> self scheme
            Command(NextOptuid),
            Command(NextOptuid),
            Command(NewChainInitActive),
            Command(Skip),              // skip Space at scheme start
            Command(Construct),         // until end of chain
            Command(PreviousOptuid),
            Command(NewChainAddOptuid),
            Command(NewChainDetach),

            Construction(NextToStored),
            Construction(Discard)          // probably unnecessary
        ];

         // "Spacify": insert spaces before all but Construction-s
        let mut scheme_spaced = vec![];
        let spaces = vec![Space; spacity];
        for content in scheme {
            match content {
                Construction(..) => {},
                _ => {
                    scheme_spaced.extend(spaces.iter());
                }

            }
            scheme_spaced.push(content);
        }

        let scheme_ouid = self.add_linear_passive_chain(&scheme_spaced);

        // Constructor, accompanying the scheme
        let constr_chain_start_ouid = self.add_new_node(Space);

        let mut nouid = constr_chain_start_ouid;

        // Copying of scheme
        nouid = self.add_new_node_to_existing(Command(SetDataOptuidFromOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NextOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NewChainInitPassive), &nouid);
        nouid = self.add_new_node_to_existing(Command(PreviousOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(Skip), &nouid);
        nouid = self.add_new_node_to_existing(Command(Replicate), &nouid);
        nouid = self.add_new_node_to_existing(Command(NewChainDetach), &nouid);

        // Constructor
        nouid = self.add_new_node_to_existing(Command(SetDataOptuidFromOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NextOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NextOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NewChainInitActive), &nouid);
        nouid = self.add_new_node_to_existing(Command(Skip), &nouid);
        nouid = self.add_new_node_to_existing(Command(Construct), &nouid);
        nouid = self.add_new_node_to_existing(Command(PreviousOptuid), &nouid);
        nouid = self.add_new_node_to_existing(Command(NewChainAddOptuid), &nouid);
        self.add_new_node_to_existing(Command(NewChainDetach), &nouid);
        // Not looped, runs only once

        let mut constr_ctrl = Controller::new();
        constr_ctrl.chain_start_optuid = constr_chain_start_ouid;
        constr_ctrl.optuids[0] = scheme_ouid;
        constr_ctrl.exec_optuid = constr_ctrl.chain_start_optuid;

        self.add_controller(constr_ctrl);
    }

}