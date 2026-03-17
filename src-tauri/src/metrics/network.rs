use sysinfo::Networks;

pub struct NetworkInfo {
    networks: Networks,
    prev_rx: u64,
    prev_tx: u64,
}

impl NetworkInfo {
    pub fn new() -> Self {
        let networks = Networks::new_with_refreshed_list();
        Self {
            networks,
            prev_rx: 0,
            prev_tx: 0,
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

        let rx_speed = total_rx.saturating_sub(self.prev_rx);
        let tx_speed = total_tx.saturating_sub(self.prev_tx);

        self.prev_rx = total_rx;
        self.prev_tx = total_tx;

        (rx_speed, tx_speed)
    }

    pub fn total(&mut self) -> (u64, u64) {
        self.networks.refresh();
        let mut total_rx = 0u64;
        let mut total_tx = 0u64;

        for (_name, data) in self.networks.iter() {
            total_rx += data.received();
            total_tx += data.transmitted();
        }

        (total_rx, total_tx)
    }
}
