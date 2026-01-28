#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn chat(name: &str) -> String {
    format!("Chatting with {}! This is a chat response from Rust!", name)
}