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

extern crate chrono;
extern crate crossterm;
extern crate emyzelium;
extern crate enum_primitive_derive;
extern crate num_traits;
extern crate rand;
extern crate serde;
extern crate serde_json;

use crossterm::style::Stylize;

use std::{
    env,
    io::{
        self,
        Write
    }
};

mod aelhometta;
mod commander;
mod serbin;

use {
    aelhometta::{
        Ælhometta,
        FORMAT_VERSION
    },
    commander::{
        Commander,
        ParseErrorPrefixise
    }
};

const PROG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PROG_DATE: &str = "2024.01.23";

fn run() -> Result<bool, String> {
    println!("{} {} ({}) {} {}",
        " Æ l h o m e t t a ".black().on_white().bold(),
        format!("v{}", PROG_VERSION).white().bold(),
        format!("{}", PROG_DATE),
        "format".dark_grey(),
        format!("v{}", FORMAT_VERSION).dark_grey().bold()
    );

    let args = env::args().collect::<Vec<String>>();
    
    let duration: Option<u64> = if args.len() > 1 {
        match args[1].parse::<u64>() {
            Ok(dur) => Some(dur),
            Err(err) => {
                return err.prefixised("duration");
            }            
        }        
    } else {
        None
    };

    print!("{}", "Loading Ælhometta... ".dark_blue());
    io::stdout().flush().unwrap_or(());
    let mut æh = match Ælhometta::load_default() {
        Ok(æh) => {
            println!("{}", "OK".dark_green().bold());
            æh
        },
        Err(err) => {
            println!("{}\n{}", format!("Cannot load Ælhometta: {}", &err).red().bold(), "Using new default one".blue().bold());
            Ælhometta::new_default()
        }
    };

    print!("{}", "Loading Commander... ".dark_blue());
    io::stdout().flush().unwrap_or(());
    let mut comm = match Commander::load_default() {
        Ok(comm) => {
            println!("{}", "OK".dark_green().bold());
            comm
            
        },
        Err(err) => {
            println!("{}\n{}", format!("Cannot load Commander: {}", &err).red().bold(), "Using new default one".blue().bold());
            Commander::new_default()
        }
    };

    let (do_save, keyint) = if duration.is_some() {
        match comm.run(&mut æh, duration) {
            Ok(kp) => (true, kp),
            Err(err) => return Err(err)
        }
    } else {
        (comm.shell(&mut æh).map_err(|err| format!("Cannot run shell: {}", &err))?, false)
    };

    print!("{}", "Saving Ælhometta... ".dark_blue());
    io::stdout().flush().unwrap_or(());
    if do_save {
        æh.save_default().map_err(|err| format!("Cannot save Ælhometta: {}", &err))?;
        println!("{}", "OK".dark_green().bold());
    } else {
        println!("{}", "CANCELLED".dark_yellow().bold());
    }

    print!("{}", "Saving Commander... ".dark_blue());
    io::stdout().flush().unwrap_or(());
    if do_save {
        comm.save_default().map_err(|err| format!("Cannot save Commander: {}", &err))?;
        println!("{}", "OK".dark_green().bold());
    } else {
        println!("{}", "CANCELLED".dark_yellow().bold());
    }

    Ok(keyint)
}

fn main() -> Result<(), i32> {
    match run() {
        Ok(keyint) => match keyint {
            false => Ok(()),
            true => Err(1)
        },
        Err(err) => {
            eprintln!("Error: {}", &err);
            Err(2)
        }
    }
}


// To avoid linker errors ("undefined reference to ...") at cross-compiling from Linux to 32-bit Windows
// Follows https://github.com/rust-lang/rust/issues/79609#issuecomment-1161560324

#[cfg(all(target_os = "windows", target_arch = "x86"))]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() {}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
#[no_mangle]
pub extern "C" fn _Unwind_RaiseException() {}