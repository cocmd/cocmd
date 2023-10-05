// use ratatui::event::{Event, KeyCode};
use ratatui::layout::{Layout, Wrap};
use ratatui::widgets::{Block, List, Paragraph, Text};
use ratatui::Terminal;

pub fn tui() {
    // Create a new Terminal instance
    let mut terminal = Terminal::new().unwrap();

    // Create a layout with two columns: package selection on the left and automations on the right
    let layout = Layout::new(LayoutDirection::Horizontal)
        .split(LayoutStyle::Percentage(30))
        .split(LayoutStyle::Percentage(70));

    // Create a list of packages on the left
    let packages = vec!["Package 1", "Package 2", "Package 3"];
    let package_list = List::new(packages)
        .block(Block::default().title("Packages").borders(true))
        .highlight_style(Style::default().fg(Color::Yellow));

    // Create a list of automations on the right
    let automations = vec![
        ("Automation 1", "Description 1"),
        ("Automation 2", "Description 2"),
        ("Automation 3", "Description 3"),
    ];

    let automation_list = List::new(automations.iter().map(|(name, _)| name))
        .block(Block::default().title("Automations").borders(true))
        .highlight_style(Style::default().fg(Color::Yellow));

    // Create a text widget to display stdout
    let stdout_text = Text::new("")
        .block(Block::default().title("Stdout").borders(true))
        .wrap(Wrap { trim: true });

    // Create a layout for the right side with automations list and stdout
    let right_layout = Layout::new(LayoutDirection::Vertical)
        .split(LayoutStyle::Percentage(80))
        .split(LayoutStyle::Percentage(20));

    // Create a layout for the entire screen
    let root_layout = Layout::new(LayoutDirection::Horizontal)
        .child(&package_list)
        .child(
            Layout::new(LayoutDirection::Vertical)
                .child(&automation_list)
                .child(&stdout_text),
        );

    // Main event loop
    loop {
        terminal.draw(|f| {
            f.render_widget(root_layout, f.size());
        });

        // match terminal.poll_event().unwrap() {
        //     Ok(Event::Key(KeyEvent {
        //         code,
        //         state,
        //         kind,
        //         modifiers,
        //     })) => {
        //         if state == KeyState::Down {
        //             match code {
        //                 KeyCode::Enter => {
        //                     // Handle Enter key press to execute automation and display stdout
        //                     // Add your code here to execute the selected automation and update the stdout_text widget.
        //                     // For simplicity, I'm leaving this part blank as it depends on your specific logic.
        //                 }
        //                 _ => (),
        //             }
        //         }
        //     }
        //     _ => (),
        // }
    }
}
