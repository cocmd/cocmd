use ratatui::widgets::{canvas::*, *};
use ratatui::{
    backend::Backend,
    layout::Alignment,
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::app::{App, AppFocus};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let area = frame.size();
    // render a list of packages on the left based on package.name() in app.packages_manager.packages
    if app.focus == AppFocus::Packages || app.focus == AppFocus::Automations {
        let chunks = Layout::default()
            .constraints([
                Constraint::Length(15),
                Constraint::Min(8),
                Constraint::Length(7),
            ])
            .direction(Direction::Horizontal)
            .split(area);

        draw_packages_list(frame, app, chunks[0]);
        draw_automations_list(frame, app, chunks[1]);
    } else {
        let automation = app
            .automations_list
            .items
            .get(app.automations_list.state.selected().unwrap())
            .unwrap()
            .clone();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
            .split(area);

        frame.render_widget(
            Paragraph::new(format!(
                "{}",
                automation
                    .content
                    .as_ref()
                    .unwrap()
                    .description
                    .as_ref()
                    .unwrap_or(&"Not provided".to_string())
            ))
            .block(
                Block::default()
                    .title(format!("{}", &automation.name))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow).bg(Color::Black))
            .alignment(Alignment::Left),
            chunks[0],
        );

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(20), Constraint::Min(1)].as_ref())
            .split(chunks[1]);

        draw_steps_list(frame, app, chunks[0]);
    }
}

fn draw_packages_list<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    let tasks: Vec<ListItem> = app
        .packages_list
        .items
        .iter()
        .map(|i| ListItem::new(vec![text::Line::from(Span::raw(i.name()))]))
        .collect();
    // set style to yellow border and text if app.focus == AppFocus::Packages
    let tasks = if app.focus == AppFocus::Packages {
        List::new(tasks)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Packages")
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ")
    } else {
        List::new(tasks)
            .block(Block::default().borders(Borders::ALL).title("Packages"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
    };

    f.render_stateful_widget(tasks, area, &mut app.packages_list.state);
}

fn draw_automations_list<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    if app.packages_list.state.selected().is_none() {
        return;
    }

    let tasks: Vec<ListItem> = app
        .automations_list
        .items
        .iter()
        .map(|i| ListItem::new(vec![text::Line::from(Span::raw(i.name.to_string()))]))
        .collect();

    let title = Span::styled(format!("Workflows").to_string(), Style::default());

    // set style to yellow border if app.focus == AppFocus::Automations
    let tasks = if app.focus == AppFocus::Automations {
        List::new(tasks)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            )
            .highlight_symbol("> ")
    } else {
        List::new(tasks)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
    };

    f.render_stateful_widget(tasks, area, &mut app.automations_list.state);
}

fn draw_steps_list<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    if app.packages_list.state.selected().is_none()
        || app.automations_list.state.selected().is_none()
    {
        return;
    }

    let tasks: Vec<ListItem> = app
        .steps_list
        .items
        .iter()
        .map(|i| ListItem::new(vec![text::Line::from(Span::raw(i.title.to_string()))]))
        .collect();

    let title = Span::styled(
        format!("Steps ({})", tasks.len()).to_string(),
        Style::default(),
    );

    // set style to yellow border if app.focus == AppFocus::Automations
    let tasks = List::new(tasks)
        .block(
            Block::default()
                .borders(Borders::NONE)
                .title(title)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(tasks, area, &mut app.steps_list.state);
}
