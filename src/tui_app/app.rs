use std::error;

use super::helpers::StatefulList;
use crate::core::models::package_config_model::Automation;
use crate::core::models::script_model::StepModel;
use crate::core::package::Package;
use crate::core::packages_manager::PackagesManager;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AppFocus {
    Packages,
    Automations,
    AutomationDetails,
    Execution,
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub focus: AppFocus,
    pub packages_manager: PackagesManager,
    pub packages_list: StatefulList<Package>,
    pub automations_list: StatefulList<Automation>,
    pub steps_list: StatefulList<StepModel>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(packages_manager: PackagesManager) -> Self {
        let items = packages_manager
            .packages
            .values()
            .map(|p| p.clone())
            .collect();
        Self {
            focus: AppFocus::Packages,
            running: true,
            packages_manager,
            packages_list: StatefulList::with_items(items),
            automations_list: StatefulList::with_items(vec![]),
            steps_list: StatefulList::with_items(vec![]),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn refresh_automations(&mut self) {
        if self.packages_list.state.selected().is_none() {
            return;
        }
        let selected_package = self
            .packages_list
            .items
            .get(self.packages_list.state.selected().unwrap())
            .unwrap();

        let settings = &self.packages_manager.settings;
        let automations = selected_package.automations(settings, Some(true));

        self.automations_list = StatefulList::with_items(automations);
    }

    pub fn refresh_steps(&mut self) {
        if self.automations_list.state.selected().is_none() {
            return;
        }
        let selected_automation = self
            .automations_list
            .items
            .get(self.automations_list.state.selected().unwrap())
            .unwrap();

        let steps = selected_automation
            .content
            .as_ref()
            .unwrap()
            .steps
            .iter()
            .map(|s| s.clone())
            .collect();

        self.steps_list = StatefulList::with_items(steps);
    }

    pub fn get_selected_package(&self) -> Option<Package> {
        if self.packages_list.items.is_empty() || self.packages_list.state.selected().is_none() {
            return None;
        }

        let selected_package = self
            .packages_list
            .items
            .get(self.packages_list.state.selected().unwrap())
            .unwrap();

        Some(selected_package.clone())
    }

    pub fn get_selected_automation(&self) -> Option<Automation> {
        if self.automations_list.items.is_empty()
            || self.automations_list.state.selected().is_none()
        {
            return None;
        }

        let selected_automation = self
            .automations_list
            .items
            .get(self.automations_list.state.selected().unwrap())
            .unwrap();

        Some(selected_automation.clone())
    }
}
