use crate::config::last_selection_dir;

use freedesktop_entry_parser::parse_entry;

use std::path::{Path, PathBuf};
use std::fs;

#[derive(Clone)]
pub struct Session {
    pub name: String,
    pub exec: String,
    pub try_exec: String,
    pub path: PathBuf,
    pub is_x11: bool
}

#[inline(always)]
pub fn push_from<T: AsRef<Path>>(sessions: &mut Vec<Session>, path: T, is_x11: bool) {
    match fs::read_dir(path) {
        Ok(dir) => {
            for file in dir {
                let path = file.unwrap().path();
                let session = get_session(path, is_x11); 
                sessions.push(session);
            }
        },
        Err(_) => return
    }
}

#[inline(always)]
pub fn push_all(sessions: &mut Vec<Session>) {
    push_from(sessions, "/usr/share/wayland-sessions", false);
    push_from(sessions, "/usr/share/xsessions", true);
}

#[inline(always)]
pub fn get_dummy() -> Session {
    return Session { 
        name: String::new(),
        exec: String::new(),
        try_exec: String::new(),
        path: PathBuf::new(),
        is_x11: false
    }
}

#[inline(always)]
pub fn is_dummy(session: &Session) -> bool {
    return session.exec.len() == 0;
}

pub fn get_session(path: PathBuf, is_x11: bool) -> Session {
    match parse_entry(&path){
        Ok(desktop) => {
            let section = desktop.section("Desktop Entry");
            let session = Session {
                name: section.attr("Name").expect("Attribute 'Name' not found").to_string(),
                exec: section.attr("Exec").expect("Attribute 'Exec' not found").trim().to_string(),
                try_exec: section.attr("TryExec").unwrap_or_else(|| {
                    return "";
                }).to_string(),
                path: path,
                is_x11: is_x11
            }; 
            return session;
        },
        Err(_) => {
            return get_dummy();
        }
    }
}

#[inline(always)]
pub fn get_recent(config_dir: &String) -> Session {
    return match fs::read_to_string(last_selection_dir(config_dir)) {
        Ok(path) => { 
            let is_x11 = path.starts_with("/usr/share/xsessions");
            let pathbuf = PathBuf::from(path);
            get_session(pathbuf, is_x11)
        },
        Err(_) => get_dummy()
    };
}