use crate::ping::Ping;

pub struct App {
    pub ping: Ping,
    pub data: Vec<(f64, f64)>,
    pub window: [f64; 2],
}

impl App {
    pub fn new() -> App {
        let mut ping = Ping::to_host("8.8.8.8");
        let mut data = Vec::<(f64, f64)>::new();
        let mut ind = 0f64;

        for _ in 0..95 {
            data.push((ind, 0f64));
            ind += 0.2f64;
        }
        ping.x = ind;
        data.extend(ping.by_ref().take(5));

        App {
            ping,
            data,
            window: [0.0, 20.0],
        }
    }

    pub fn on_tick(&mut self) {
        for _ in 0..5 {
            self.data.remove(0);
        }
        self.data.extend(self.ping.by_ref().take(5));

        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}