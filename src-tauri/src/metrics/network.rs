use std::time::Instant;

use sysinfo::Networks;

pub struct NetworkInfo {
    networks: Networks,
    prev_rx: u64,
    prev_tx: u64,
    last_refresh: Instant,
    total_rx: u64,
    total_tx: u64,
}

impl NetworkInfo {
    pub fn new() -> Self {
        let networks = Networks::new_with_refreshed_list();
        let mut info = Self {
            networks,
            prev_rx: 0,
            prev_tx: 0,
            last_refresh: Instant::now(),
            total_rx: 0,
            total_tx: 0,
        };
        info.prime();
        info
    }

    /// Prime the previous values with current totals to avoid initial spike
    fn prime(&mut self) {
        self.networks.refresh();
        for (_name, data) in self.networks.iter() {
            self.prev_rx += data.received();
            self.prev_tx += data.transmitted();
        }
    }

    pub fn speeds(&mut self) -> (u64, u64) {
        self.networks.refresh();
        let mut total_rx = 0u64;
        let mut total_tx = 0u64;

        for (_name, data) in self.networks.iter() {
            total_rx += data.received();
            total_tx += data.transmitted();
        }

        let elapsed = self.last_refresh.elapsed().as_secs_f64();
        let elapsed = if elapsed > 0.0 { elapsed } else { 1.0 };

        let rx_speed = (total_rx.saturating_sub(self.prev_rx) as f64 / elapsed) as u64;
        let tx_speed = (total_tx.saturating_sub(self.prev_tx) as f64 / elapsed) as u64;

        self.prev_rx = total_rx;
        self.prev_tx = total_tx;
        self.last_refresh = Instant::now();
        self.total_rx = total_rx;
        self.total_tx = total_tx;

        (rx_speed, tx_speed)
    }

    pub fn total(&mut self) -> (u64, u64) {
        (self.total_rx, self.total_tx)
    }
}
