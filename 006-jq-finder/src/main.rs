use anyhow::Result;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use jq_finder::app::run_app;
use jq_finder::jq::Jq;
use jq_finder::state::State;
use tui::{backend::CrosstermBackend, Terminal};

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
fn main() -> Result<()> {
    let opt = Opt::from_args();

    // setup jq
    let json = get_json(&opt.json_file)?;
    let jq = Jq::new(&opt.jq_path, &json);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = State::default();
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
