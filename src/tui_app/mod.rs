/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui_struct;

/// Event handler.
pub mod handler;

pub mod helpers;

use std::io;

use app::{App, AppFocus, AppResult};
use event::{Event, EventHandler};
use handler::handle_key_events;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tui_struct::Tui;

use crate::core::packages_manager::PackagesManager;

pub fn tui_runner(packages_manager: PackagesManager) -> AppResult<Option<String>> {
    // Create an application.
    let mut app = App::new(packages_manager);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        if app.focus == AppFocus::Execution {
            let package = app
                .packages_list
                .items
                .get(app.packages_list.state.selected().unwrap())
                .unwrap()
                .clone();

            let automation = app
                .automations_list
                .items
                .get(app.automations_list.state.selected().unwrap())
                .unwrap()
                .clone();
            tui.events.stop();
            tui.exit()?;
            return Ok(Some(format!("{}.{}", package.name(), automation.name)));
        }

        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::Terminate => {
                tui.exit()?;
                return Ok(None);
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(None)
}
