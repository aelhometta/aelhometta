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

use emyzelium::Efunguz;

use std::{
    collections::HashSet,
    time::{
        SystemTime,
        UNIX_EPOCH
    }
};

use super::{
    OtherPeer,
    Ælhometta
};

const KEY_Z85_LEN: usize = 40;

const ETALE_TITLE: &str = "i64s";

const ERR_NOT_EXPOSED: &str = "Peer not exposed";
const ERR_ALREADY_EXPOSED: &str = "Peer already exposed";

impl Ælhometta {
    fn peer_reconnect_all_others(&mut self) {
        if let Some(ref mut efunguz) = self.efunguz {
            for op in & self.other_peers {
                if op.publickey().len() == KEY_Z85_LEN {
                    if let Ok(eh) = efunguz.add_ehypha(& op.publickey(), & op.onion(), op.port()) {
                        let _ = eh.add_etale(ETALE_TITLE);
                    }
                }
            }
        }
    }

    pub fn peer_share_size(&mut self, size: usize) -> Result<(), String> {
        self.share_size = size.min(self.ether_integers.len());
        Ok(())
    }

    pub fn peer_share_interval(&mut self, interval: i64) -> Result<(), String> {
        if interval >= 0 {
            self.share_interval = interval;
            Ok(())
        } else {
            Err(String::from("Interval must be non-negative"))
        }
    }

    pub fn peer_share_now(&mut self) -> Result<(), String> {
        if let Some(ref mut ef) = self.efunguz {
            ef.emit_etale("", & vec!["i64s".as_bytes().to_vec(), "64-bit signed integers".as_bytes().to_vec()]);
            ef.emit_etale(ETALE_TITLE, & vec![self.ether_integers[..self.share_size].iter().map(|&i| {
                i.to_le_bytes()
            }).flatten().collect::<Vec<u8>>()]);
            self.ut_last_share = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |dur| dur.as_micros()) as i64;
            Ok(())
        } else {
            Err(String::from(ERR_NOT_EXPOSED))
        }
    }

    pub fn peer_update(&mut self) {
        if let Some(ref mut ef) = self.efunguz {
            ef.update();
            for op in &mut self.other_peers {
                if let Some(eh) = ef.get_ehypha(& op.publickey) {
                    if let Some(et) = eh.get_etale(ETALE_TITLE) {
                        if et.parts().len() == 1 {
                            let ether_bytes = & et.parts()[0];
                            if ether_bytes.len() & 7 == 0 {
                                let l = ether_bytes.len() >> 3;
                                op.ether_integers = Vec::with_capacity(l);
                                let mut buf = [0u8; 8];
                                let mut offs: usize = 0;
                                for _ in 0..l {
                                    buf.copy_from_slice(& ether_bytes[offs..(offs + 8)]);
                                    op.ether_integers.push(i64::from_le_bytes(buf));
                                    offs += 8;
                                }
                                op.ut_last_update = et.t_in();
                            }
                        }
                    }
                }
            }

            if self.share_interval > 0 {
                let ut = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |dur| dur.as_micros()) as i64;
        
                if ut - self.ut_last_share > self.share_interval {
                    let _ = self.peer_share_now();
                }
            }
        }
    }

    pub fn peer_secret(&mut self, secretkey: &str) -> Result<(), String> {
        if let None = self.efunguz {
            self.secretkey = secretkey.to_string();
            Ok(())
        } else {
            Err(String::from(ERR_ALREADY_EXPOSED))
        }
    }

    pub fn peer_port(&mut self, port: u16) -> Result<(), String> {
        if let None = self.efunguz {
            self.port = port;
            Ok(())
        } else {
            Err(String::from(ERR_ALREADY_EXPOSED))
        }
    }

    pub fn peer_torport(&mut self, port: u16) -> Result<(), String> {
        if let None = self.efunguz {
            self.torproxy_port = port;
            Ok(())
        } else {
            Err(String::from(ERR_ALREADY_EXPOSED))
        }
    }

    pub fn peer_torhost(&mut self, host: &str) -> Result<(), String> {
        if let None = self.efunguz {
            self.torproxy_host = host.to_string();
            Ok(())
        } else {
            Err(String::from(ERR_ALREADY_EXPOSED))
        }
    }

    pub fn peer_expose(&mut self) -> Result<(), String> {
        if self.efunguz.is_none() {
            if self.secretkey.len() == KEY_Z85_LEN {
                self.efunguz = Some(Efunguz::new(
                    self.secretkey.as_str(), & self.whitelist, self.port, self.torproxy_port, self.torproxy_host.as_str()
                ));
                self.exposed = true;
                self.peer_reconnect_all_others();
                Ok(())
            } else {
                Err(format!("Wrong secret key length: {}, must be {}", self.secretkey.len(), KEY_Z85_LEN))
            }
        } else {
            Err(String::from(ERR_ALREADY_EXPOSED))
        }
    }

    pub fn peer_repose(&mut self) -> Result<(), String> {
        if self.efunguz.is_some() {
            // "now" becomes "before"
            self.in_permitted_before_num += self.efunguz.as_ref().unwrap().in_permitted_num();
            self.in_attempted_before_num += self.efunguz.as_ref().unwrap().in_attempted_num();

            self.efunguz = None;
            self.exposed = false;
            Ok(())
        } else {
            Err(String::from("Peer not exposed"))
        }
    }

    pub fn peer_connect(&mut self, publickey: &str, onion: &str, port: u16) -> Result<(), String> {
        if publickey.len() == KEY_Z85_LEN {
            self.other_peers.push(OtherPeer::new(publickey, onion, port)); // register "intention to connect"
            if let Some(ref mut efunguz) = self.efunguz { // may be None when this peer is not exposed
                match efunguz.add_ehypha(publickey, onion, port) {
                    Ok(eh) => {
                        match eh.add_etale(ETALE_TITLE) {
                            Ok(_) => {
                                Ok(())
                            },
                            Err(err) => Err(format!("Cannot subscribe to data: {}", err))
                        }
                    },
                    Err(err) => Err(format!("Cannot connect to peer: {}", err))
                }
            } else { // may be None when this peer is not exposed
                Ok(())
            }
        } else {
            Err(format!("Wrong public key length: {}, must be {}", publickey.len(), KEY_Z85_LEN))
        }
    }

    pub fn peer_disconnect(&mut self, publickey: &str) -> Result<(), String> {
        if publickey.len() == KEY_Z85_LEN {
            let mut found = false;
            for (i, op) in &mut self.other_peers.iter().enumerate() {
                if op.publickey == publickey {
                    found = true;
                    self.other_peers.remove(i);
                    // Fix peer indices of all controllers
                    for (_, ctrl) in self.controllers.iter_mut() {
                        if ctrl.i_peer > 1 + i {
                            ctrl.i_peer -= 1;
                        } else if ctrl.i_peer == 1 + i {
                            ctrl.i_peer = 0; // "reset"
                        }
                    }
                    if let Some(ref mut efunguz) = self.efunguz { // may be None when this peer is not exposed
                        match efunguz.del_ehypha(publickey) {
                            Ok(_) => {},
                            Err(err) => {
                                return Err(format!("Cannot disconnect from peer: {}", err));
                            }
                        }
                    }
                    break;
                }
            }
            match found {
                true => Ok(()),
                false => Err(String::from("That peer has not been registered"))
            }
        } else {
            Err(format!("Wrong public key length: {}, must be {}", publickey.len(), KEY_Z85_LEN))
        }
    }

    pub fn peer_whitelist_add(&mut self, publickey: &str) -> Result<(), String> {
        if publickey.len() == KEY_Z85_LEN {
            if !self.whitelist.contains(publickey) {
                self.whitelist.insert(publickey.to_string());
                if let Some(ref mut efunguz) = self.efunguz { // may be None when this peer is not exposed
                    efunguz.add_whitelist_publickeys(& HashSet::from([publickey.to_string()]));
                }
                Ok(())
            } else {
                Err(String::from("Public key already whitelisted"))
            }
        } else {
            Err(format!("Wrong public key length: {}, must be {}", publickey.len(), KEY_Z85_LEN))
        }
    }

    pub fn peer_whitelist_del(&mut self, publickey: &str) -> Result<(), String> {
        if publickey.len() == KEY_Z85_LEN {
            if self.whitelist.contains(publickey) {
                self.whitelist.remove(publickey);
                if let Some(ref mut efunguz) = self.efunguz { // may be None when this peer is not exposed
                    efunguz.del_whitelist_publickeys(& HashSet::from([publickey.to_string()]));
                }
                Ok(())
            } else {
                Err(String::from("Public key not whitelisted"))
            }
        } else {
            Err(format!("Wrong public key length: {}, must be {}", publickey.len(), KEY_Z85_LEN))
        }
    }

    pub fn peer_whitelist_clear(&mut self) {
        self.whitelist.clear();
        if let Some(ref mut efunguz) = self.efunguz { // may be None when this peer is not exposed
            efunguz.clear_whitelist_publickeys();
        }
    }

}