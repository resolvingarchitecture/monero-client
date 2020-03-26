
use log::{info};

use ra_common::models::{Packet};

pub struct MoneroClient {

}

impl MoneroClient {
    pub fn new() -> MoneroClient {
        MoneroClient {

        }
    }
    pub fn init(&mut self) {
        info!("{}","Initializing Monero Client...");

    }
    pub fn handle(&mut self, packet: &mut Packet) {
        info!("Handling incoming packet id={}",packet.id);

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
