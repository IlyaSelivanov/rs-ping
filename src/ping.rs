use std::{net::IpAddr, time::Duration};

use ping_rs::PingOptions;

#[derive(Clone)]
pub struct Ping {
    pub x: f64,
    pub addres: IpAddr,
    pub data: [u8; 4],
    pub timeout: Duration,
    pub options: PingOptions,
}

impl Ping {
    pub fn to_host(addr: &str) -> Self {
        let addres = addr.parse().unwrap();

        Ping {
            x: 0f64,
            addres,
            data: [1, 2, 3, 4],
            timeout: Duration::from_secs(1),
            options: ping_rs::PingOptions {
                ttl: 128,
                dont_fragment: true,
            },
        }
    }

    fn ping(&mut self) -> Option<(f64, f64)> {
        let result = ping_rs::send_ping(
            &self.addres,
            self.timeout,
            &self.data[..4],
            Some(&self.options),
        );
        self.x += 0.2f64;
        match result {
            Ok(reply) => Some((self.x, reply.rtt as f64)),
            Err(_) => Some((self.x, 0f64)),
        }
    }
}

impl Iterator for Ping {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        self.ping()
    }
}
