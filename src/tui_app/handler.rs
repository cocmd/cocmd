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
                app.refresh_steps();
            }
        }
        KeyCode::Down => {
            if app.focus == AppFocus::Packages {
                app.packages_list.next();
                app.refresh_automations();
            } else if app.focus == AppFocus::Automations {
                app.automations_list.next();
                app.refresh_steps();
            }
        }

        KeyCode::Right => {
            if app.focus == AppFocus::Packages && app.packages_list.state.selected().is_some() {
                app.focus = AppFocus::Automations;
            } else if app.focus == AppFocus::Automations {
                app.focus = AppFocus::AutomationDetails;
            }
        }

        KeyCode::Left => {
            if app.focus == AppFocus::Automations {
                app.focus = AppFocus::Packages;
                app.refresh_steps();
            } else if app.focus == AppFocus::AutomationDetails {
                app.focus = AppFocus::Automations;
            }
        }

        KeyCode::Enter => {
            if app.focus == AppFocus::Packages && app.packages_list.state.selected().is_some() {
                app.focus = AppFocus::Automations;
            } else if app.focus == AppFocus::Automations
                && app.automations_list.state.selected().is_some()
            {
                app.focus = AppFocus::AutomationDetails;
            } else if app.focus == AppFocus::AutomationDetails {
                app.focus = AppFocus::Execution;
            }
        }

        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
