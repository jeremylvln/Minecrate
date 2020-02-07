use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use env_logger::Env;
use log::{info, warn};
use ctrlc;
use network::connection::ConnectionHandler;

pub mod config;
pub mod server;
pub mod packet_consumers;

use config::Config;
use server::MinecraftServer;

fn main() {
    println!("Welcome to Minecrate!");
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    
    info!("Loading configuration...");
    let config = Config::from_path("server.toml").unwrap_or(Config::default());
    let mut server = MinecraftServer::new(config);
    let mut connection = ConnectionHandler::new();
    let run = Arc::new(AtomicBool::new(true));
    let run_cpy = run.clone();

    ctrlc::set_handler(move || {
        warn!("Received interruption signal...");
        run_cpy.store(false, Ordering::SeqCst);
    }).expect("Failed to set interrupt handler");

    connection.listen(run.clone(), &server.config.host.clone(), server.config.port, |stream, packet| {
        packet_consumers::packet_process(&mut server, stream, packet)
    }).expect("Failed to start the server");

    println!("Goodbye!");
}