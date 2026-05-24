use crate::session::Session;
use std::fs;
use std::path::PathBuf;

fn get_snapshots_dir() -> Result<PathBuf, String> {
    let home = std::env::var("HOME")
        .map_err(|_| "Could not determine home directory".to_string())?;
    
    let snapshots_dir = PathBuf::from(home).join("ghost-snapshots");
    
    fs::create_dir_all(&snapshots_dir)
        .map_err(|e| format!("Failed to create snapshots directory: {}", e))?;
    
    Ok(snapshots_dir)
}

pub fn serialize_session(session: &Session) -> Result<(), String> {
    let json_data = serde_json::to_string_pretty(session)
        .map_err(|e| format!("Failed to serialize: {}", e))?;

    let snapshots_dir = get_snapshots_dir()?;
    let filepath = snapshots_dir.join(format!("{}.json", session.name));

    fs::write(filepath, json_data)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

pub fn deserialize_session(name: &str) -> Result<Session, String> {
    let snapshots_dir = get_snapshots_dir()?;
    let filepath = snapshots_dir.join(format!("{}.json", name));

    let json_data = fs::read_to_string(filepath)
        .map_err(|e| format!("Failed to find session '{}': {}", name, e))?;

    let session: Session = serde_json::from_str(&json_data)
        .map_err(|e| format!("Failed to parse session: {}", e))?;

    Ok(session)
}

pub fn delete_session(name: &str) -> Result<(), String> {
    let snapshots_dir = get_snapshots_dir()?;
    let filepath = snapshots_dir.join(format!("{}.json", name));

    fs::remove_file(filepath)
        .map_err(|e| format!("Failed to delete session '{}': {}", name, e))?;

    Ok(())
}
