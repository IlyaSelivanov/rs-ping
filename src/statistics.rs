pub const BUFFER_SIZE: usize = 10;

#[derive(Debug, Clone)]
pub struct Statistics<T> {
    buffer_size: usize,
    data: Vec<T>,
    averages: Vec<f32>,
}

impl Statistics<u32> {
    pub fn new() -> Self {
        let buffer_size = BUFFER_SIZE;
        let data: Vec<u32> = Vec::new();
        let averages: Vec<f32> = Vec::new();

        Statistics {
            buffer_size,
            data,
            averages,
        }
    }

    fn flush(&mut self) {
        let average: f32 = self.average();
        self.data = Vec::new();
        self.averages.push(average);
    }

    pub fn push(&mut self, value: u32) {
        if self.data.len() >= self.buffer_size {
            self.flush();
        }

        self.data.push(value);
    }

    pub fn average(&self) -> f32 {
        self.data.iter().sum::<u32>() as f32 / self.data.len() as f32
    }

    pub fn last_average(&self) -> Option<&f32> {
        self.averages.iter().rev().next()
    }
}
