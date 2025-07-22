mod sessions;
mod config;
mod cli;

use sessions::{Session, is_dummy};
use std::process::Command;
use std::fs;

fn main() {
    let mut sessions: Vec<Session> = Vec::new();
    let config_dir: String = config::config_dir();

    let _ = fs::create_dir_all(&config_dir);
    
    sessions::push_recent(&mut sessions, &config_dir);
    sessions::push_all(&mut sessions);

    cli::print_prompt();
    cli::print_sessions(&sessions);

    let session = cli::get_input(&sessions);
    if is_dummy(&session) {
        println!("No session selected, exiting to TTY.");
        return;
    }

    config::set_recent(&session, &config_dir);

    if config::try_exec(&session.try_exec) {
        Command::new("/bin/sh")
                .arg("-c")
                .arg(session.exec)
                .output()
                .expect("Failed to start selection.");
    }

}