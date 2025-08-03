use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
};

use crate::model::{Model, View};

/// Main view function that renders the UI based on current model state
pub fn view(model: &Model, frame: &mut Frame) {
    let area = frame.area();
    
    // Create main layout with header and content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Render header with tabs
    render_header(model, frame, chunks[0]);
    
    // Render current view content
    match model.current_view {
        View::Status => render_status_view(model, frame, chunks[1]),
        View::Log => render_log_view(model, frame, chunks[1]),
        View::Branches => render_branches_view(model, frame, chunks[1]),
        View::Files => render_files_view(model, frame, chunks[1]),
    }
}

/// Render the header with navigation tabs
fn render_header(model: &Model, frame: &mut Frame, area: Rect) {
    let tab_titles = vec!["Status", "Log", "Branches", "Files"];
    let selected_tab = match model.current_view {
        View::Status => 0,
        View::Log => 1,
        View::Branches => 2,
        View::Files => 3,
    };

    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("ggit"))
        .select(selected_tab)
        .style(Style::default().white())
        .highlight_style(Style::default().yellow().bold());

    frame.render_widget(tabs, area);
}

/// Render the status view (git status equivalent)
fn render_status_view(model: &Model, frame: &mut Frame, area: Rect) {
    let content = if model.has_repository() {
        format!(
            "Repository: {}\nWorking Directory: {}\n\nStatus information will go here...",
            model.repository.as_ref().unwrap().path().display(),
            model.current_path
        )
    } else {
        format!(
            "No Git repository found in: {}\n\nNavigate to a Git repository or initialize one.",
            model.current_path
        )
    };

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

/// Render the log view (git log equivalent)
fn render_log_view(model: &Model, frame: &mut Frame, area: Rect) {
    let items = if model.has_repository() {
        vec![
            ListItem::new("commit abc123 - Add initial commit"),
            ListItem::new("commit def456 - Update README"),
            ListItem::new("commit ghi789 - Fix bug in parser"),
        ]
    } else {
        vec![ListItem::new("No repository available")]
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Commit Log"))
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black))
        .highlight_symbol("→ ");

    frame.render_stateful_widget(list, area, &mut ratatui::widgets::ListState::default().with_selected(Some(model.selected_index)));
}

/// Render the branches view
fn render_branches_view(model: &Model, frame: &mut Frame, area: Rect) {
    let items = if model.has_repository() {
        vec![
            ListItem::new("* main"),
            ListItem::new("  feature/new-ui"),
            ListItem::new("  bugfix/parser-error"),
        ]
    } else {
        vec![ListItem::new("No repository available")]
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Branches"))
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black))
        .highlight_symbol("→ ");

    frame.render_stateful_widget(list, area, &mut ratatui::widgets::ListState::default().with_selected(Some(model.selected_index)));
}

/// Render the files view (working tree files)
fn render_files_view(model: &Model, frame: &mut Frame, area: Rect) {
    let items = if model.has_repository() {
        vec![
            ListItem::new("M  src/main.rs"),
            ListItem::new("A  src/model.rs"),
            ListItem::new("?? temp.txt"),
        ]
    } else {
        vec![ListItem::new("No repository available")]
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Files"))
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black))
        .highlight_symbol("→ ");

    frame.render_stateful_widget(list, area, &mut ratatui::widgets::ListState::default().with_selected(Some(model.selected_index)));
}
