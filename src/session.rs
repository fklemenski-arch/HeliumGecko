use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::io::{BufRead, BufReader, Write};
use tokio::task;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    pub url: String,
    pub opacity: f32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub windows: Vec<WindowState>,
    pub last_update: String,
}

impl Default for SessionState {
    fn default() -> Self {
        Self { windows: vec![], last_update: String::new() }
    }
}

impl SessionState {
    pub fn load_or_default() -> Self {
        let path = session_path();
        if path.exists() {
            let data = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    pub fn save(&self) {
        let path = session_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(&path, json).ok();
    }
}

fn session_path() -> PathBuf {
    let proj = directories::ProjectDirs::from("com", "heliumgecko", "Helium").unwrap();
    proj.data_dir().join("session.json")
}

// Simple TCP‑based IPC for synchronising settings across windows
pub async fn start_ipc(state: Arc<Mutex<SessionState>>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("IPC bind failed");
    let addr = listener.local_addr().unwrap();
    // Write address to a known file so other windows can connect
    let ipc_file = directories::ProjectDirs::from("com", "heliumgecko", "Helium")
        .unwrap()
        .runtime_dir()
        .unwrap_or_else(|| std::env::temp_dir())
        .join("helium_ipc_port");
    fs::write(&ipc_file, addr.to_string()).ok();

    task::spawn_blocking(move || {
        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let mut buf = String::new();
                let reader = BufReader::new(&mut stream);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        buf.push_str(&line);
                    }
                }
                if let Ok(mut state) = state.lock() {
                    if let Ok(new) = serde_json::from_str::<SessionState>(&buf) {
                        *state = new;
                        state.save();
                    }
                }
            }
        }
    });
}

pub fn notify_other_windows(state: &SessionState) {
    let ipc_file = directories::ProjectDirs::from("com", "heliumgecko", "Helium")
        .unwrap()
        .runtime_dir()
        .unwrap_or_else(|| std::env::temp_dir())
        .join("helium_ipc_port");
    if let Ok(port_str) = fs::read_to_string(&ipc_file) {
        if let Ok(mut stream) = TcpStream::connect(port_str.trim()) {
            let json = serde_json::to_string(state).unwrap();
            stream.write_all(json.as_bytes()).ok();
        }
    }
}
