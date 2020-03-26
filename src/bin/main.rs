extern crate log;
extern crate simple_logger;

use log::{trace,info};
use monero_client::MoneroClient;

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting Monero Client Daemon...");
    let mut monero_client = MoneroClient::new();
    monero_client.init();
    trace!("Monero Client Daemon Stopped.");
}
