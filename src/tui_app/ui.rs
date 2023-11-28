use ratatui::widgets::*;
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
    let chunks = if app.focus == AppFocus::Packages {
        Layout::default()
            .constraints([
                Constraint::Min(15),
                Constraint::Min(15),
                Constraint::Min(20),
            ])
            .direction(Direction::Horizontal)
            .split(area)
    } else if app.focus == AppFocus::Automations {
        Layout::default()
            .constraints([
                Constraint::Max(10),
                Constraint::Min(15),
                Constraint::Min(35),
            ])
            .direction(Direction::Horizontal)
            .split(area)
    } else {
        Layout::default()
            .constraints([
                Constraint::Max(10),
                Constraint::Max(10),
                Constraint::Min(36),
            ])
            .direction(Direction::Horizontal)
            .split(area)
    };

    draw_packages_list(frame, app, chunks[0]);
    draw_automations_list(frame, app, chunks[1]);
    draw_steps_list(frame, app, chunks[2]);
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

    let title = Span::styled("Workflows".to_string().to_string(), Style::default());

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
    let selected_automation = app.get_selected_automation();

    if selected_automation.is_none() {
        return;
    }

    let package = app.get_selected_package().unwrap();
    let automation = selected_automation.unwrap();

    let binding = "No description provided".to_string();

    let name = format!("{}.{}", package.name(), automation.name);
    // Render a bordered block that is yellow if app.focus == AppFocus::AutomationDetails
    // in the block write the automation name and description
    // and the steps list with a title and description to each step

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .title(name.to_string())
        .title_style(Style::default().fg(Color::Yellow))
        .border_type(BorderType::Rounded);

    let block = if app.focus == AppFocus::AutomationDetails {
        block
    } else {
        block.border_style(Style::default())
    };

    let inner = block.inner(area);

    f.render_widget(block, area);

    let chunks = Layout::default()
        .constraints([
            Constraint::Max(4),
            Constraint::Length(1),
            Constraint::Min(20),
            // another chunk in the bottom for a button to run the automation
            Constraint::Length(5),
        ])
        .direction(Direction::Vertical)
        .split(inner);

    let description = automation
        .content
        .as_ref()
        .unwrap()
        .description
        .as_ref()
        .unwrap_or(&binding);

    let description = "Description: \n".to_string() + description;

    // render description in chunks[0]
    let description = Paragraph::new(description.to_string())
        .block(Block::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(description, chunks[0]);

    // render in chunks[1] a string "Steps:"
    let steps = format!(
        "Steps ({}):",
        automation.content.as_ref().unwrap().steps.len()
    );
    let steps = Paragraph::new(steps)
        .block(Block::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(steps, chunks[1]);

    // add a list of the steps and descriptions
    let tasks: Vec<ListItem> = automation
        .content
        .as_ref()
        .unwrap()
        .steps
        .iter()
        .map(|i| {
            let text = vec![text::Span::raw(format!("- {}", i.get_title()))];
            ListItem::new(vec![text::Line::from(text)])
        })
        .collect();
    // render a non-stateful list in chunks[1]
    let tasks = List::new(tasks).block(Block::default());

    f.render_widget(tasks, chunks[2]);

    // render a button in chunks[3] with the text "Run↳"
    // the button should be centralized in the area, occupying the entire width and hight and have a yellow background
    // the button background should all be with yellow color
    // the text should be in black
    // the text should be center in width and height
    // if the button is clicked, run the automation

    let button = Paragraph::new("\nPress Enter ↳ to run")
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::Black).bg(Color::Yellow));

    f.render_widget(button, chunks[3]);
}
