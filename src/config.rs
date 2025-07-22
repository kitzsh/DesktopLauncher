use crate::sessions::Session;

use std::{fs, env};

#[inline(always)]
pub fn config_dir() -> String {
    let home = env::var("HOME").unwrap();
    let cratename = "desktoplauncher";
    let path = format!("{home}/.config/{cratename}");
    return path.to_string();
}

#[inline(always)]
pub fn last_selection_dir(config_dir: &String) -> String {
    return format!("{config_dir}/last-selection");
}

#[inline(always)]
pub fn set_recent(session: &Session, config_dir: &String) {
    let _ = fs::write(last_selection_dir(config_dir), session.path.display().to_string());
}

#[inline(always)]
pub fn try_exec(path: &String) -> bool {
    if path.len() == 0 {
        return true;
    }

    if fs::exists(path).expect("TryExec could not be accessed.") {
        return true;
    }

    return env::var("PATH").and_then(|pathvar| {
        Ok(pathvar.split(":").map(|dir| 
            format!("{}/{}", dir, path)
        ).any(|file| 
            fs::exists(file).expect("TryExec could not be accessed.")
        ))
    }).expect("TryExec could not be accessed.");
}