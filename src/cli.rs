use crate::sessions::{Session, is_dummy, get_dummy};

use std::num::IntErrorKind;
use std::io::Write;
use std::io;

#[inline(always)]
pub fn print_prompt(recent: &Session) {
    if is_dummy(&recent) {
        println!("Select a desktop environment. Leave blank to return to TTY.\n");
    }
    else {
        println!("Select a desktop environment. Leave blank to return use the last selected environment ({}).\n", recent.name);
    }
}

#[inline(always)]
pub fn print_sessions(sessions: &Vec<Session>) {
    println!("0) Exit to TTY");
    for (i, session) in sessions.iter().enumerate() {
        println!("{}) {}", i+1, session.name);
    }
}

#[inline(always)]
pub fn get_input(sessions: &Vec<Session>, recent: &Session) -> Session {
    assert!(sessions.len() > 0);

    let maximum: usize = sessions.len();

    loop {
        print!("Select (0-{})> ", maximum);
        io::stdout().flush().unwrap();

        let mut selection_string = String::new();
        io::stdin().read_line(&mut selection_string).expect("Invalid input");

        match selection_string.trim().parse::<usize>() {
            Ok(selection) => {
                if selection > maximum || selection < 0 {
                    println!("Selection out of bounds. Try again.");
                    continue;
                }

                if selection == 0 {
                    return get_dummy();
                }

                return sessions[selection - 1].clone();
            },
            Err(e) => match e.kind() {
                IntErrorKind::InvalidDigit => {
                    println!("Input is not a valid integer. Try again.");
                    continue;
                },
                IntErrorKind::Empty => {
                    return recent.clone();
                },
                _ => panic!("Unhandled exception while parsing selection: {e:?}")
            }
        }
    }
}