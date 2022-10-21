use anyhow::Result;
use std::{
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
    thread::{self},
    time::{Duration, Instant},
};
use structopt::StructOpt;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use jq_finder::app::App;
use jq_finder::jq::Jq;
use jq_finder::ui::ui;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

#[derive(StructOpt)]
struct Opt {
    #[structopt(help = "jq path", long = "jq-path", default_value = "jq")]
    jq_path: String,
    #[structopt(help = "json file path", long = "json-file")]
    json_file: String,
}

fn get_json(json_file: &str) -> Result<String> {
    let reader = BufReader::new(std::fs::File::open(json_file)?);
    Ok(reader
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .join("\n"))
}

fn run_app<B: Backend>(mut app: App, terminal: &mut Terminal<B>, jq: Jq) -> Result<()> {
    // first filter
    let jq_output = jq.execute(&app.filter)?;
    app.update_output(jq_output)?;

    // filter loop
    let jq = Arc::new(jq);
    let app = Arc::new(Mutex::new(app));
    let last_input_at = Arc::new(Mutex::new(Instant::now()));
    loop {
        terminal.draw(|f| {
            let app = &app.lock().unwrap();
            ui(f, &app)
        })?;

        if !event::poll(Duration::from_millis(50))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            let mut locked_app = app.lock().unwrap();
            match key.code {
                KeyCode::Char(c) => {
                    if c == 'c' && key.modifiers == KeyModifiers::CONTROL {
                        return Ok(());
                    }
                    locked_app.filter.push(c);
                }
                KeyCode::Backspace => {
                    locked_app.filter.pop();
                }
                _ => {}
            }

            {
                let mut locked_last_input_at = last_input_at.lock().unwrap();
                *locked_last_input_at = Instant::now();
            }

            // filter json
            let app = Arc::clone(&app);
            let last_input_at = Arc::clone(&last_input_at);
            let jq = Arc::clone(&jq);
            let _ = thread::spawn(move || {
                let delay = Duration::from_millis(200);
                thread::sleep(delay);
                let last_input_at = last_input_at.lock().unwrap();
                if last_input_at.elapsed() >= delay {
                    let mut app = app.lock().unwrap();
                    if let Ok(output) = jq.execute(&app.filter) {
                        let _ = app.update_output(output);
                    }
                }
            });
        }
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    // create jq
    let json = get_json(&opt.json_file)?;
    let jq = Jq::new(&opt.jq_path, &json);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(app, &mut terminal, jq);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
