use ctrlc;
use env_logger::Env;
use log::{info, warn};
use network::connection::ConnectionHandler;
use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub mod config;
pub mod packet_consumers;
pub mod server;

use config::Config;
use server::MinecraftServer;

fn main() {
    println!("Welcome to Minecrate!");
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    info!("Loading configuration...");
    let config = Config::from_path("server.toml").unwrap_or_else(|_| Config::default());
    let server = RefCell::new(MinecraftServer::new(config));
    let mut connection = ConnectionHandler::new();
    let run = Arc::new(AtomicBool::new(true));
    let run_cpy = run.clone();

    ctrlc::set_handler(move || {
        warn!("Received interruption signal...");
        run_cpy.store(false, Ordering::SeqCst);
    })
    .expect("Failed to set interrupt handler");

    let (host, port) = {
        let server = server.borrow();

        (server.config.host.clone(), server.config.port)
    };

    connection
        .listen(
            run,
            &host,
            port,
            || {
                server.borrow_mut().tick();
            },
            |stream, packet| {
                packet_consumers::packet_process(&mut server.borrow_mut(), stream, packet)
            },
        )
        .expect("Failed to start the server");

    println!("Goodbye!");
}
