mod session;

use std::fs;

fn serialize_session(session: &Session) -> Result<(), String> {
    let json_data = serde_json::to_string_pretty(session)
        .map_err(|e| format!("Failed to serialize: {}", e))?;

    let filename = format!("{}.json", session.name);

    fs::write(filename, json_data)
        .map_err(|e| format!("Failed to write file: {e}", e))?;

    Ok(())
}

fn deserialize_session(name: &str) -> Result<Session, String> {
    let filename = format!("{}.json", name);

    let json_data = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to find session '{}': {}", name, e))?;

    let session: Session = serde_json::from_str(&json_data)
        .map_err(|e| format!("Failed to parse session: {}", e))?;

    Ok(session)
}
