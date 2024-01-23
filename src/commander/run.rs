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
    cursor::{
        self,
        MoveTo,
    },
    event::{
        self,
        Event,
        KeyCode,
        KeyModifiers
    },
    style::{
        Color,
        Colors,
        Print,
        SetColors,
        Stylize
    },
    queue,
    terminal,
    ExecutableCommand
};

use std::{
    collections::{BTreeMap, VecDeque},
    io::{
        self,
        stdout,
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
        Command,
        Construction,
        Hexly
    },
    super::{
        Commander,
        Settings
    }
};

const FREQ_BRANCHES_MAIN_CAPTION: &str = "BranchesMain";
const FREQ_BRANCHES_ALT_CAPTION: &str = "BranchesAlt";

fn sec_to_hms_str(mut seconds: u64) -> String {
    let s = seconds % 60;
    seconds /= 60;
    let m = seconds % 60;
    let h = seconds / 60;
    format!("{}:{:02}:{:02}", h, m, s)
}

fn print_symbol(sym: &str, x: i16, y: i16, fc: Color, bc: Color) {
    let _ = queue!(stdout(),
        MoveTo(x as u16, y as u16),
        SetColors(Colors::new(fc, bc)),
        Print(sym)
    );
}

fn clear_drawn_frequencies(comm_count_len: usize) -> Result<(), String> {
    let _ = queue!(stdout(),
        cursor::SavePosition
    );

    let (term_width, _) = terminal::size().map_err(|err| err.to_string())?; 
    let term_width = term_width as i16;

    // Only 1st line, the rest will be overdrawn by new window
    for i in 0..comm_count_len {
        let x = term_width - ((comm_count_len - i) as i16);
        if x >= 0 {
            print_symbol(" ", x, 0, Color::Grey, Color::Reset);
        }
    }

    let _ = queue!(stdout(),
        cursor::RestorePosition
    );

    Ok(())
}

fn draw_frequencies(comm_count: &Vec<(Command, u128)>, cons_count: &Vec<(Construction, u128)>, branches_main_count: u128, branches_alt_count: u128, i_sel_comm: usize, i_sel_cons: usize, settings: &Settings) -> Result<(), String> {
    // Sanity checks
    if (comm_count.len() == 0) || (cons_count.len() == 0) {
        return Err(String::from("Count is empty yet"));
    }

    let _ = queue!(stdout(),
        cursor::SavePosition
    );

    // Welcome to spaghetti of hand-crafted TUI...

    let (term_width, term_height) = terminal::size().map_err(|err| err.to_string())?; 
    let (term_width, term_height) = (term_width as i16, term_height as i16);
    let freq_window_height = (term_height - (settings.freqs_window_margin as i16) - 1).max(1);
    let freq_window_width = comm_count.len() as i16;

    let cons_chart_height = cons_count.len() as i16;

    // Frame vertical line
    let x = term_width - freq_window_width - 1;
    if x >= 0 {
        for y in 0..freq_window_height {
            print_symbol("│", x, y, Color::DarkGrey, Color::Reset);
        }
    }

    // Selected Command
    let i_sel_comm = i_sel_comm % comm_count.len(); // sanity clamp
    let sel_comm_chars: Vec<String> = format!("{:?}", comm_count[i_sel_comm].0).chars().map(|ch| ch.to_string()).collect();
    let sel_comm_count_chars: Vec<String> = format!("{}", comm_count[i_sel_comm].1).chars().map(|ch| ch.to_string()).collect();

    for x in 0..freq_window_width {
        let (sym, text_color) = if (x as usize) < sel_comm_chars.len() {
            (sel_comm_chars[x as usize].as_str(), Color::White)
        } else if (x as usize) + sel_comm_count_chars.len() >= (freq_window_width as usize) {
            (sel_comm_count_chars[(x as usize) + sel_comm_count_chars.len() - (freq_window_width as usize)].as_str(), Color::DarkYellow)
        } else {
            (" ", Color::Grey)
        };
        print_symbol(sym, term_width - freq_window_width + x, 0, text_color, Color::Reset);
    }

    // Command-s
    let bar_max_height = (freq_window_height - (settings.freqs_comm_str_len as i16) - 2 - cons_chart_height).max(0);
    let max_count = comm_count.iter().fold(0, |acc, x| acc.max(x.1)).max(1);
    for (i, (comm, count)) in comm_count.iter().enumerate() {
        let x = term_width - freq_window_width + (i as i16);
        if x >= 0 {
            let comm_chars = format!("{:?}", *comm).chars().map(|ch| ch.to_string()).collect::<Vec<String>>();
            let l = comm_chars.len();
            let color_bright = if i == i_sel_comm {
                Color::White
            } else {
                Color::Yellow
            };
            let color_dark_back = if i == i_sel_comm {
                Color::DarkGrey
            } else {
                Color::Reset
            };
            for y in 0..settings.freqs_comm_str_len {
                let sym = if l <= settings.freqs_comm_str_len { // full command name
                    if y < l {
                        & comm_chars[y]
                    } else {
                        "·"
                    }
                } else { // beginning, "...", end
                    if y < (settings.freqs_comm_str_len >> 1) {
                        & comm_chars[y]
                    } else if y > (settings.freqs_comm_str_len >> 1) {
                        & comm_chars[l + y - settings.freqs_comm_str_len]
                    } else {
                        "⁞"
                    }
                };
                print_symbol(sym, x, 1 + (y as i16), [color_bright, Color::Black][i & 1], [color_dark_back, color_bright][i & 1]);
            }
            let h = ((*count) * ((bar_max_height as u128) << 1) / max_count) as i16;
            let color_bar = if i == i_sel_comm {
                Color::DarkYellow
            } else {
                [Color::Blue, Color::DarkBlue][i & 1]
            };
            for y in 0..bar_max_height {
                let sym = if h >= ((y << 1) + 2) {
                    "█"
                } else if h >= ((y << 1) + 1) {
                    "▀"
                } else {
                    " "
                };
                print_symbol(sym, x, 1 + (settings.freqs_comm_str_len as i16) + y, color_bar, Color::Reset);
            }
            // Frame horizontal line
            print_symbol("─", x, freq_window_height, Color::DarkGrey, Color::Reset);
        }
    }

    // Branches (main/alternative choices ratio)
    let w = (branches_main_count * (freq_window_width as u128) / (branches_main_count + branches_alt_count).max(1)) as i16;
    let freq_branch_main_chars: Vec<String> = format!("{}={}", FREQ_BRANCHES_MAIN_CAPTION, branches_main_count).chars().map(|ch| ch.to_string()).collect();
    let freq_branch_alt_chars: Vec<String> = format!("{}={}", FREQ_BRANCHES_ALT_CAPTION, branches_alt_count).chars().map(|ch| ch.to_string()).collect();
    for x in 0..freq_window_width {
        let sym = if (x as usize) < freq_branch_main_chars.len() {
            freq_branch_main_chars[x as usize].as_str()
        } else if (x as usize) + freq_branch_alt_chars.len() >= (freq_window_width as usize) {
            freq_branch_alt_chars[(x as usize) + freq_branch_alt_chars.len() - (freq_window_width as usize)].as_str()
        } else {
            " "
        };
        let bc = if w >= (x + 1) {
            Color::DarkGreen
        } else {
            Color::DarkMagenta
        };
        print_symbol(sym, term_width - freq_window_width + x, 1 + (settings.freqs_comm_str_len as i16) + bar_max_height, Color::Yellow, bc);
    }

    // Construction-s
    let bar_max_width = (freq_window_width - (settings.freqs_cons_str_len as i16)).max(0);
    let max_count = cons_count.iter().fold(0, |acc, x| acc.max(x.1)).max(1);
    let i_sel_cons = i_sel_cons % cons_count.len(); // sanity clamp
    for (i, (cons, count)) in cons_count.iter().enumerate() {
        let cons_chars = format!("{:?}", *cons).chars().map(|ch| ch.to_string()).collect::<Vec<String>>();
        let l = cons_chars.len();
        let color_bright = if i == i_sel_cons {
            Color::White
        } else {
            Color::Magenta
        };
        let color_dark_back = if i == i_sel_cons {
            Color::DarkGrey
        } else {
            Color::Reset
        };
        for x in 0..settings.freqs_cons_str_len {
            let sym = if x < l {
                & cons_chars[x]
            } else {
                "·"
            };
            print_symbol(sym, term_width - freq_window_width + (x as i16), freq_window_height - cons_chart_height + (i as i16), [color_bright, Color::Black][i & 1], [color_dark_back, color_bright][i & 1]);
        }
        let w = ((*count) * (bar_max_width as u128) / max_count) as i16;
        let cons_count_chars = format!("{}", *count).chars().map(|ch| ch.to_string()).collect::<Vec<String>>();
        for x in 0..bar_max_width {
            let bc = if i == i_sel_cons {
                if w >= (x + 1) {
                    Color::DarkYellow
                } else {
                    Color::Reset
                }
            } else {
                if w >= (x + 1) {
                    [Color::Blue, Color::DarkBlue][i & 1]
                } else {
                    Color::Reset
                }                
            };
            let sym = if i == i_sel_cons {
                if (x as usize) + cons_count_chars.len() >= (bar_max_width as usize) {
                    cons_count_chars[(x as usize) + cons_count_chars.len() - (bar_max_width as usize)].as_str()
                } else {
                    " "
                }
            } else {
                " "
            };            
            print_symbol(sym, term_width - freq_window_width + (settings.freqs_cons_str_len as i16) + x, freq_window_height - cons_chart_height + (i as i16), Color::Yellow, bc);
        }
    }

    // Frame corner (very important)
    print_symbol("└", term_width - freq_window_width - 1, freq_window_height, Color::DarkGrey, Color::Reset);

    let _ = queue!(stdout(),
        cursor::RestorePosition
    );

    Ok(())
}

impl Commander {
    pub fn run(&mut self, æh: &mut Ælhometta, duration: Option<u64>) -> Result<bool, String> {
        terminal::enable_raw_mode().map_err(|err| err.to_string())?;
        let _ = io::stdout().execute(cursor::Hide);

        if self.settings.show_freqs {
            print!("{}\r\n", "Press ← → ↑ ↓ to select content, any other key to stop".dark_grey());
        } else {
            print!("{}\r\n", "Press any key to stop".dark_grey());
        }        

        let t_start = Instant::now();
        let mut last_t_elapsed: u64 = 0;

        let mut last_comm_totals = BTreeMap::<Command, u128>::new();
        for (command, count) in æh.commands_count() {
            last_comm_totals.insert(*command, *count);
        }
        let mut comm_counts = VecDeque::<Vec<(Command, u128)>>::with_capacity(self.settings.freqs_interval);
        let mut comm_sum_count = Vec::<(Command, u128)>::new();

        let mut last_cons_totals = BTreeMap::<Construction, u128>::new();
        for (construction, count) in æh.constructions_count() {
            last_cons_totals.insert(*construction, *count);
        }
        let mut cons_counts = VecDeque::<Vec<(Construction, u128)>>::with_capacity(self.settings.freqs_interval);
        let mut cons_sum_count = Vec::<(Construction, u128)>::new();

        let mut last_branches_main_count = æh.branches_main_count();
        let mut branches_main_counts = VecDeque::<u128>::with_capacity(self.settings.freqs_interval);
        let mut branches_main_sum_count = 0;

        let mut last_branches_alt_count = æh.branches_alt_count();
        let mut branches_alt_counts = VecDeque::<u128>::with_capacity(self.settings.freqs_interval);
        let mut branches_alt_sum_count = 0;

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

                    if self.settings.show_freqs {
                        clear_drawn_frequencies(last_comm_totals.len()).unwrap_or(());
                    }

                    Self::print_state(æh, false);
                    print!("\r\n");

                    if self.settings.show_freqs {
                        // For Command-s...
                        let mut comm_totals = BTreeMap::<Command, u128>::new();
                        for (command, count) in æh.commands_count() {
                            comm_totals.insert(*command, *count);
                        }
                        let l = comm_totals.len();
                        let mut comm_count_last_sec = Vec::<(Command, u128)>::with_capacity(l);
                        for (command, count) in & comm_totals {
                            comm_count_last_sec.push((*command, (*count) - last_comm_totals.get(command).unwrap_or(&0)));
                        }
                        last_comm_totals = comm_totals;
                        comm_counts.push_back(comm_count_last_sec);
                        if comm_counts.len() > self.settings.freqs_interval {
                            comm_counts.pop_front();
                        }
                        comm_sum_count = (0..l).map(|i| {
                            (comm_counts[0][i].0, comm_counts.iter().map(|cc| cc[i].1).sum())
                        }).collect();

                        // ...and the same for Construction-s
                        let mut cons_totals = BTreeMap::<Construction, u128>::new();
                        for (construction, count) in æh.constructions_count() {
                            cons_totals.insert(*construction, *count);
                        }
                        let l = cons_totals.len();
                        let mut cons_count_last_sec = Vec::<(Construction, u128)>::with_capacity(l);
                        for (construction, count) in & cons_totals {
                            cons_count_last_sec.push((*construction, (*count) - last_cons_totals.get(construction).unwrap_or(&0)));
                        }
                        last_cons_totals = cons_totals;
                        cons_counts.push_back(cons_count_last_sec);
                        if cons_counts.len() > self.settings.freqs_interval {
                            cons_counts.pop_front();
                        }
                        cons_sum_count = (0..l).map(|i| {
                            (cons_counts[0][i].0, cons_counts.iter().map(|cc| cc[i].1).sum())
                        }).collect();
                        // TODO: factor out the duplication

                        let branches_main_count = æh.branches_main_count();
                        branches_main_counts.push_back(branches_main_count - last_branches_main_count);
                        if branches_main_counts.len() > self.settings.freqs_interval {
                            branches_main_counts.pop_front();
                        }
                        last_branches_main_count = branches_main_count;
                        branches_main_sum_count = branches_main_counts.iter().sum();                        

                        let branches_alt_count = æh.branches_alt_count();
                        branches_alt_counts.push_back(branches_alt_count - last_branches_alt_count);
                        if branches_alt_counts.len() > self.settings.freqs_interval {
                            branches_alt_counts.pop_front();
                        }
                        last_branches_alt_count = branches_alt_count;
                        branches_alt_sum_count = branches_alt_counts.iter().sum();

                        draw_frequencies(&comm_sum_count, &cons_sum_count, branches_main_sum_count, branches_alt_sum_count, self.selections.i_command, self.selections.i_construction, & self.settings).unwrap_or(());
                    }

                    io::stdout().flush().unwrap_or(());
                }
            }

            while let Ok(true) = event::poll(Duration::from_secs(0)) {
                if let Ok(Event::Key(kev)) = event::read() {
                    if self.settings.show_freqs {
                        let key_code = kev.code;
                        let step: usize = if kev.modifiers.contains(KeyModifiers::SHIFT) {
                            8
                        } else {
                            1
                        };
                        match key_code {
                            KeyCode::Left => {
                                self.selections.i_command = (self.selections.i_command + last_comm_totals.len() - step) % last_comm_totals.len();
                                draw_frequencies(&comm_sum_count, &cons_sum_count, branches_main_sum_count, branches_alt_sum_count, self.selections.i_command, self.selections.i_construction, & self.settings).unwrap_or(());
                                io::stdout().flush().unwrap_or(());
                            },
                            KeyCode::Right => {
                                self.selections.i_command = (self.selections.i_command + step) % last_comm_totals.len();
                                draw_frequencies(&comm_sum_count, &cons_sum_count, branches_main_sum_count, branches_alt_sum_count, self.selections.i_command, self.selections.i_construction, & self.settings).unwrap_or(());
                                io::stdout().flush().unwrap_or(());
                            },
                            KeyCode::Up => {
                                self.selections.i_construction = (self.selections.i_construction + last_cons_totals.len() - 1) % last_cons_totals.len();
                                draw_frequencies(&comm_sum_count, &cons_sum_count, branches_main_sum_count, branches_alt_sum_count, self.selections.i_command, self.selections.i_construction, & self.settings).unwrap_or(());
                                io::stdout().flush().unwrap_or(());
                            },
                            KeyCode::Down => {
                                self.selections.i_construction = (self.selections.i_construction + 1) % last_cons_totals.len();
                                draw_frequencies(&comm_sum_count, &cons_sum_count, branches_main_sum_count, branches_alt_sum_count, self.selections.i_command, self.selections.i_construction, & self.settings).unwrap_or(());
                                io::stdout().flush().unwrap_or(());
                            },
                            _ => {
                                break 'ext true;
                            }
                        }  
                    } else {
                        break 'ext true;
                    }                  
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

        let _ = stdout().execute(cursor::Show);
        terminal::disable_raw_mode().map_err(|err| err.to_string())?;

        Ok(keypress)
    }

}