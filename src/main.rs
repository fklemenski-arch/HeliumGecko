use clap::Parser;
use log::info;
use std::sync::{Arc, Mutex};

mod config;
mod engine;
mod overlay;
mod platform;
mod session;
mod shortcuts;
mod window_manager;

/// Helium Gecko – always-on-top transparent browser
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// URL to open on launch
    #[arg(short, long, default_value = "https://www.youtube.com")]
    url: String,
    /// Use a specific config file
    #[arg(short, long)]
    config: Option<String>,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    let config = config::Config::load(args.config.as_deref());
    let session_state = Arc::new(Mutex::new(session::SessionState::load_or_default()));

    // Start IPC listener for multi-window settings sync
    let _ipc_handle = tokio::spawn(session::start_ipc(session_state.clone()));

    // Create first window with the given URL
    let mut wm = window_manager::WindowManager::new(config, session_state);
    wm.open_window(&args.url);

    info!("Helium Gecko running – press Ctrl+Q to quit");
    // main loop runs inside window_manager
}
