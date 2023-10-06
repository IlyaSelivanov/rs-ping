use std::{
    io,
    net::IpAddr,
    thread,
    time::{self, Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ping_rs::PingOptions;
use ratatui::{prelude::*, widgets::*};
use statistics::Statistics;

use crate::statistics::BUFFER_SIZE;

mod statistics;

#[derive(Clone)]
struct Ping {
    x: f64,
    addres: IpAddr,
    data: [u8; 4],
    timeout: Duration,
    options: PingOptions,
}

impl Ping {
    fn to_host(addr: &str) -> Self {
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

struct App {
    ping: Ping,
    data: Vec<(f64, f64)>,
    window: [f64; 2],
}

impl App {
    fn new() -> App {
        let mut ping = Ping::to_host("8.8.8.8");

        let mut data = Vec::<(f64, f64)>::new();

        let mut ind = 0f64;
        for _ in 0..95 {
            data.push((ind, 0f64));
            ind += 0.2f64;
        }
        ping.x = ind;
        data.extend(ping.by_ref().take(5));

        // let data = ping.by_ref().take(200).collect::<Vec<(f64, f64)>>();

        App {
            ping,
            data,
            window: [0.0, 20.0],
        }
    }

    fn on_tick(&mut self) {
        for _ in 0..5 {
            self.data.remove(0);
        }
        self.data.extend(self.ping.by_ref().take(5));

        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}

fn main() -> Result<(), io::Error> {
    let use_chart: bool = true;

    if !use_chart {
        ping();

        Ok(())
    } else {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // create app and run it
        let tick_rate = Duration::from_secs(1);
        let app = App::new();
        let res = run_app(&mut terminal, app, tick_rate);

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{err:?}");
        }

        Ok(())
    }
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 1)].as_ref())
        .split(size);
    let x_labels = vec![
        Span::styled(
            format!("{}", app.window[0]),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("{}", (app.window[0] + app.window[1]) / 2.0)),
        Span::styled(
            format!("{}", app.window[1]),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ];
    let datasets = vec![Dataset::default()
        .name("ping")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .graph_type(GraphType::Line)
        .data(&app.data)];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title("Chart".cyan().bold())
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .labels(x_labels)
                .bounds(app.window),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .labels(vec!["0".bold(), "500".bold()])
                .bounds([0.0, 500.0]),
        );
    f.render_widget(chart, chunks[0]);
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
