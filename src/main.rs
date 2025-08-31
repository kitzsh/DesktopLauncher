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
    
    let recent = sessions::get_recent(&config_dir);
    sessions::push_all(&mut sessions);

    cli::print_prompt(&recent);
    cli::print_sessions(&sessions);

    let session = cli::get_input(&sessions, &recent);
    if is_dummy(&session) {
        println!("No session selected, exiting to TTY.");
        return;
    }

    config::set_recent(&session, &config_dir);

    if config::try_exec(&session.try_exec) {
        let exec: String;
        if session.is_x11 {
            // uncommon edge case, but good to have here anyway
            if session.exec.contains(char::is_whitespace) {
                exec = format!("TMP=$(mktemp /tmp/desktoplauncher.XXXX)\necho \"rm $TMP && {}\" > $TMP\nexec startx $TMP", session.exec);
            }
            else {
                exec = format!("exec startx $(which {})", session.exec);
            }
        }
        else {
            exec = format!("exec {}", session.exec);
        }

        println!("Starting {}...", session.name);

        Command::new("/bin/sh")
                .arg("-c")
                .arg(exec)
                .spawn()
                .expect("Failed to start selection.");
    }
}