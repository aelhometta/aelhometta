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

use chrono::prelude::*;

use crossterm::{
    cursor,
    event::{
        self,
        Event
    },
    style::Stylize,
    terminal,
    ExecutableCommand
};

use std::{
    io::{
        self,
        Write
    },
    time::{
        Duration,
        Instant,
        SystemTime,
        UNIX_EPOCH
    }
};

use {
    crate::aelhometta::{
        Ælhometta,
        Hexly
    },
    super::Commander
};

fn sec_to_hms_str(mut seconds: u64) -> String {
    let s = seconds % 60;
    seconds /= 60;
    let m = seconds % 60;
    let h = seconds / 60;
    format!("{}:{:02}:{:02}", h, m, s)
}

impl Commander {
    pub fn run(&mut self, æh: &mut Ælhometta, duration: Option<u64>) -> Result<bool, String> {
        terminal::enable_raw_mode().map_err(|err| err.to_string())?;
        let _ = io::stdout().execute(cursor::Hide);

        print!("{}\r\n", "Press any key to stop".dark_grey());

        let t_start = Instant::now();
        let mut last_t_elapsed: u64 = 0;

        let keypress = 'ext: loop {
            let tick_data = æh.tick(&None);

            let t_elapsed = t_start.elapsed().as_secs();   

            if self.settings.show_ticks || (t_elapsed > last_t_elapsed) {
                if self.settings.show_abs_time {
                    print!("{} ", format!("[{} UTC]", NaiveDateTime::from_timestamp_micros(
                        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |dur| dur.as_micros() as i64)
                    ).unwrap_or_default().format("%Y.%m.%d %a %H:%M:%S")).dark_green());
                } else {
                    print!("{}", format!("[{}", sec_to_hms_str(t_elapsed)).dark_green());
                    if let Some(dur) = duration {
                        print!(" {} ", "/".dark_grey());
                        print!("{}", sec_to_hms_str(dur).dark_green());
                    }
                    print!("{} ", "]".dark_green());
                }
            }         

            if self.settings.show_ticks {
                last_t_elapsed = t_elapsed;
                print!("{}{}{}{}{}{}{}{}{}{}{}\r\n",
                    "Age ".dark_blue(),
                    format!("{}", æh.age()).blue(),
                    " : ".dark_grey(),
                    "Ctrl ".dark_cyan(),
                    tick_data.controller_optuid.hexly().cyan(),
                    " | ".dark_grey(),
                    "Node ".dark_magenta(),
                    tick_data.exec_optuid.hexly().magenta(),
                    " | ".dark_grey(),
                    "Cont ".dark_yellow(),
                    match tick_data.exec_optcontent {
                        Some(content) => format!("{:?}", content),
                        None => format!("×")
                    }.yellow()
                );
                io::stdout().flush().unwrap_or(());
            } else {
                if t_elapsed > last_t_elapsed {
                    last_t_elapsed = t_elapsed;
                    Self::print_state(æh, false);
                    print!("\r\n");
                    io::stdout().flush().unwrap_or(());
                }
            }

            while let Ok(true) = event::poll(Duration::from_secs(0)) {
                if let Ok(Event::Key(..)) = event::read() {
                    break 'ext true;
                }
            }
/*
            if æh.num_controllers() == 0 {
                print!("\r\n\r\n{}", "NO CONTROLLERS LEFT".dark_yellow().bold());
                break;
            }
*/
            if let Some(dur) = duration {
                if t_elapsed >= dur {
                    break 'ext false;
                }
            }
        };

        let _ = io::stdout().execute(cursor::Show);
        terminal::disable_raw_mode().map_err(|err| err.to_string())?;

        Ok(keypress)
    }

}