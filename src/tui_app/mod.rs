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

use app::{App, AppResult};
use event::{Event, EventHandler};
use handler::handle_key_events;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tui_struct::Tui;

use crate::core::packages_manager::PackagesManager;

pub fn tui_runner(packages_manager: PackagesManager) -> AppResult<()> {
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
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
