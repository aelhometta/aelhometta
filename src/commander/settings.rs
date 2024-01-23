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

use super::Commander;

impl Commander {
    fn print_show_ticks(&self) {
        println!("{}{}", format!("{:24}", "show_ticks").dark_yellow(), format!("{}", self.settings.show_ticks).dark_yellow().bold());
    }

    fn print_show_abs_time(&self) {
        println!("{}{}", format!("{:24}", "show_abs_time").dark_yellow(), format!("{}", self.settings.show_abs_time).dark_yellow().bold());
    }

    fn print_sequence_def_limit(&self) {
        println!("{}{}", format!("{:24}", "sequence_def_limit").dark_blue(), format!("{}", self.settings.sequence_def_limit).blue());
    }

    fn print_show_freqs(&self) {
        println!("{}{}", format!("{:24}", "show_freqs").dark_yellow(), format!("{}", self.settings.show_freqs).dark_yellow().bold());
    }

    fn print_freqs_interval(&self) {
        println!("{}{}", format!("{:24}", "freqs_interval").dark_green(), format!("{}", self.settings.freqs_interval).green());
    }

    fn print_freqs_window_margin(&self) {
        println!("{}{}", format!("{:24}", "freqs_window_margin").dark_blue(), format!("{}", self.settings.freqs_window_margin).blue());
    }

    fn print_freqs_comm_str_len(&self) {
        println!("{}{}", format!("{:24}", "freqs_comm_str_len").dark_blue(), format!("{}", self.settings.freqs_comm_str_len).blue());
    }

    fn print_freqs_cons_str_len(&self) {
        println!("{}{}", format!("{:24}", "freqs_cons_str_len").dark_blue(), format!("{}", self.settings.freqs_cons_str_len).blue());
    }

    pub fn settings(&self, setting: &str) -> Result<(), String> {
        match setting {
            "" => {
                self.print_show_abs_time();
                self.print_show_ticks();
                self.print_sequence_def_limit();
                self.print_show_freqs();
                self.print_freqs_interval();
                self.print_freqs_window_margin();
                self.print_freqs_comm_str_len();
                self.print_freqs_cons_str_len();
                Ok(())
            },
            "show_abs_time" => {
                self.print_show_abs_time();
                Ok(())
            }
            "show_ticks" => {
                self.print_show_ticks();
                Ok(())
            },
            "sequence_def_limit" => {
                self.print_sequence_def_limit();
                Ok(())
            },
            "show_freqs" => {
                self.print_show_freqs();
                Ok(())
            },
            "freqs_interval" => {
                self.print_freqs_interval();
                Ok(())
            },
            "freqs_window_margin" => {
                self.print_freqs_window_margin();
                Ok(())
            },
            "freqs_comm_str_len" => {
                self.print_freqs_comm_str_len();
                Ok(())
            },
            "freqs_cons_str_len" => {
                self.print_freqs_cons_str_len();
                Ok(())
            },
            _ => {
                Err(format!("Unknown setting"))
            }
        }
    }

}