use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::{App, AppFocus, AppResult};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            if app.focus == AppFocus::Execution {
                app.focus = AppFocus::Automations;
            } else {
                app.quit();
            }
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Up => {
            if app.focus == AppFocus::Packages {
                app.packages_list.previous();
                app.refresh_automations();
            } else if app.focus == AppFocus::Automations {
                app.automations_list.previous();
            }
        }
        KeyCode::Down => {
            if app.focus == AppFocus::Packages {
                app.packages_list.next();
                app.refresh_automations();
            } else if app.focus == AppFocus::Automations {
                app.automations_list.next();
            }
        }

        KeyCode::Right => {
            if app.focus == AppFocus::Packages {
                app.focus = AppFocus::Automations;
            }
        }

        KeyCode::Left => {
            if app.focus == AppFocus::Automations {
                app.focus = AppFocus::Packages;
            }
        }

        KeyCode::Enter => {
            if app.focus == AppFocus::Automations {
                app.focus = AppFocus::Execution;
                app.refresh_steps();
            }
        }

        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
