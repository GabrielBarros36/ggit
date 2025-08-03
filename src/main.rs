mod model;
mod update;
mod view;

use std::{io, time::Duration};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

use model::{Model, View};
use update::{update, Message};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize application state
    let mut model = Model::new().unwrap_or_default();

    // Main application loop
    let result = run_app(&mut terminal, &mut model);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    model: &mut Model,
) -> io::Result<()> {
    loop {
        // Render the current view
        terminal.draw(|f| view::view(model, f))?;

        // Handle events and process messages
        if let Some(message) = handle_event()? {
            let mut current_msg = Some(message);
            
            // Process update chain
            while let Some(msg) = current_msg {
                current_msg = update(model, msg);
                
                // Check if we should exit
                if model.running_state == model::RunningState::Done {
                    return Ok(());
                }
            }
        }
    }
}

fn handle_event() -> io::Result<Option<Message>> {
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                return Ok(match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => Some(Message::Quit),
                    KeyCode::Char('1') => Some(Message::SwitchView(View::Status)),
                    KeyCode::Char('2') => Some(Message::SwitchView(View::Log)),
                    KeyCode::Char('3') => Some(Message::SwitchView(View::Branches)),
                    KeyCode::Char('4') => Some(Message::SwitchView(View::Files)),
                    KeyCode::Up | KeyCode::Char('k') => Some(Message::SelectUp),
                    KeyCode::Down | KeyCode::Char('j') => Some(Message::SelectDown),
                    KeyCode::Home | KeyCode::Char('g') => Some(Message::SelectFirst),
                    KeyCode::End | KeyCode::Char('G') => Some(Message::SelectLast),
                    KeyCode::F(5) | KeyCode::Char('r') => Some(Message::Refresh),
                    _ => None,
                });
            }
        }
    }
    Ok(None)
}
