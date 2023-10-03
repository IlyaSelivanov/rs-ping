use std::{
    thread,
    time::{self, Duration},
};

#[derive(Debug, Clone)]
struct Statistics<T> {
    data: Vec<T>,
}

impl Statistics<u32> {
    fn new() -> Self {
        let data: Vec<u32> = Vec::new();

        Statistics { data }
    }

    fn push(&mut self, value: u32) {
        self.data.push(value);
    }

    fn average(&self) -> f32 {
        self.data.iter().sum::<u32>() as f32 / self.data.len() as f32
    }
}

fn main() {
    let addr = "8.8.8.8".parse().unwrap();
    let data = [1, 2, 3, 4]; // ping data
    let timeout = Duration::from_secs(1);
    let options = ping_rs::PingOptions {
        ttl: 128,
        dont_fragment: true,
    };
    let mut statistics = Statistics::new();

    loop {
        let result = ping_rs::send_ping(&addr, timeout, &data, Some(&options));
        match result {
            Ok(reply) => {
                println!(
                    "Reply from {}: bytes={} time={}ms TTL={}",
                    reply.address,
                    data.len(),
                    reply.rtt,
                    options.ttl
                );

                statistics.push(reply.rtt);
            }
            Err(e) => println!("{:?}", e),
        }

        println!("Average rtt={:.2}ms", statistics.average());

        let ten_millis = time::Duration::from_secs(2);
        thread::sleep(ten_millis);
    }
}
