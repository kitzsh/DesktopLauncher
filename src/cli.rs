use crate::sessions::{Session, is_dummy, get_dummy_session};

use std::num::IntErrorKind;
use std::io::Write;
use std::io;

#[inline(always)]
pub fn print_prompt() {
    println!("Select a desktop environment. Leave blank to return to TTY.\n");
}

#[inline(always)]
pub fn print_sessions(sessions: &Vec<Session>) {
    for (i, session) in sessions.iter().enumerate() {
        if !is_dummy(session){
            let name = &session.name;
            println!("{i}) {}", if i == 0 {
                format!("Previous session ({})", name)
            } else {
                name.to_string()
            });
        }
    }
}

#[inline(always)]
pub fn get_input(sessions: &Vec<Session>) -> Session {
    assert!(sessions.len() > 0);

    let minimum = if is_dummy(&sessions[0]) {1} else {0};
    let maximum = sessions.len() - 1;

    loop {
        print!("Select ({}-{})> ", minimum, maximum);
        io::stdout().flush().unwrap();

        let mut selection_string = String::new();
        io::stdin().read_line(&mut selection_string).expect("Invalid input");

        match selection_string.trim().parse::<usize>() {
            Ok(selection) => {
                if selection > maximum || selection < minimum {
                    println!("Selection out of bounds. Try again.");
                    continue;
                }

                return sessions[selection].clone();
            },
            Err(e) => match e.kind() {
                IntErrorKind::InvalidDigit => {
                    println!("Input is not a valid integer. Try again.");
                    continue;
                },
                IntErrorKind::Empty => {
                    return get_dummy_session();
                },
                _ => panic!("Unhandled exception while parsing selection: {e:?}")
            }
        }
    }
}