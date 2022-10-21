use anyhow::Result;
use std::{
    sync::{Arc, Mutex},
    thread::{self},
    time::{Duration, Instant},
};

use crate::jq::Jq;
use crate::state::State;
use crate::ui::ui;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use tui::{backend::Backend, Terminal};

pub fn run_app<B: Backend>(mut state: State, terminal: &mut Terminal<B>, jq: Jq) -> Result<()> {
    // first filter
    let jq_output = jq.execute(&state.filter)?;
    state.update_output(jq_output)?;

    // event loop
    let min_draw_interval = Duration::from_millis(50);
    let state = Arc::new(Mutex::new(state));
    let jq = Arc::new(Mutex::new(jq));
    let last_input_at = Arc::new(Mutex::new(Instant::now()));
    loop {
        terminal.draw(|f| {
            let state = state.lock().unwrap();
            ui(f, &state)
        })?;

        if !event::poll(min_draw_interval)? {
            continue;
        }

        // キー入力を受け取り、jqコマンドを実行する
        // pollの結果がtrueであれば event::read()はノンブロッキングになる
        if let Event::Key(key) = event::read()? {
            let mut locked_state = state.lock().unwrap();
            let mut updated = false;
            match key.code {
                KeyCode::Char(c) => {
                    if c == 'c' && key.modifiers == KeyModifiers::CONTROL {
                        return Ok(());
                    }
                    updated = true;
                    locked_state.filter.push(c);
                }
                KeyCode::Backspace => {
                    updated = true;
                    locked_state.filter.pop();
                }
                _ => {}
            }

            if updated {
                {
                    let mut locked_last_input_at = last_input_at.lock().unwrap();
                    *locked_last_input_at = Instant::now();
                }

                // filter json
                let state = Arc::clone(&state);
                let jq = Arc::clone(&jq);
                let last_input_at = Arc::clone(&last_input_at);
                let _ = thread::spawn(move || {
                    let delay = Duration::from_millis(200);
                    thread::sleep(delay);
                    let last_input_at = last_input_at.lock().unwrap();
                    if last_input_at.elapsed() >= delay {
                        let mut state = state.lock().unwrap();
                        let jq = jq.lock().unwrap();
                        if let Ok(output) = jq.execute(&state.filter) {
                            let _ = state.update_output(output);
                        }
                    }
                });
            }
        }
    }
}
