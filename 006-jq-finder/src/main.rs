use anyhow::Result;
use std::io::{BufRead, BufReader, Stdin};
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

fn run_app<B: Backend>(mut app: App, terminal: &mut Terminal<B>, jq: &Jq) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => {
                    // filter json
                    let jq_output = jq.execute(&app.filter)?;
                    app.output = if jq_output.status.success() {
                        String::from_utf8(jq_output.stdout)
                    } else {
                        String::from_utf8(jq_output.stderr)
                    }?;
                }
                KeyCode::Char(c) => {
                    if c == 'c' && key.modifiers == KeyModifiers::CONTROL {
                        return Ok(());
                    }
                    app.filter.push(c);
                }
                KeyCode::Backspace => {
                    app.filter.pop();
                }
                _ => {}
            }
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
    let res = run_app(app, &mut terminal, &jq);

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
