use std::{
    io, thread,
    time::{self, Duration},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};
use statistics::Statistics;

use crate::statistics::BUFFER_SIZE;

mod statistics;

fn main() -> Result<(), io::Error> {
    let use_chart: bool = true;

    if !use_chart {
        ping();
    } else {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        thread::spawn(|| loop {
            event::read();
        });

        thread::sleep(Duration::from_millis(5000));

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
    }

    Ok(())
}

fn ping() {
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

        match statistics.last_average() {
            Some(avg) => {
                println!(
                    "Average rtt according to last {} pings is {:.2}ms",
                    BUFFER_SIZE, avg
                );
            }
            None => println!(
                "Average rtt according to last {} pings is 0.00ms",
                BUFFER_SIZE
            ),
        }

        let ten_millis = time::Duration::from_secs(2);
        thread::sleep(ten_millis);

        println!(" ");
    }
}