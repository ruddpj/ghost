use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::System;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub(crate) name: String,
    dir: String,
    shell: String,
    time: u64
}

pub fn save_session(name: String) -> Result<Session, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let pid = sysinfo::get_current_pid()
        .map_err(|_| "Could not determine own PID.".to_string())?;

    let proc = sys.process(pid)
        .ok_or("Could not find process.")?;

    let p_pid = proc.parent()
        .ok_or("Could not find parent PID.")?;

    let p_proc = sys.process(p_pid)
        .ok_or("Parent process vanished.")?;

    let dir = p_proc.cwd()
        .ok_or("Could not determine parent working directory.")?
        .to_string_lossy()
        .to_string();
    let shell = p_proc.exe()
        .ok_or("Could not determine parent shell executable.")?
        .to_string_lossy()
        .to_string();
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "Time moved backwards")?
        .as_secs();

    Ok(Session {
        name,
        dir,
        shell,
        time,
    })
}

pub fn load_session(session: Session) {
    println!("Restoring session: {}...", session.name);

    let mut child = Command::new(&session.shell);

    child.current_dir(&session.dir);

    let status = child.spawn()
        .expect("Failed to start shell.")
        .wait()
        .expect("Shell crashed or interrupted.");

    println!("Session ended with status: {}", status);
}
