use crate::{prelude::*, components::button::ButtonColor};
use super::state::*;

impl ListPage {
    pub fn add_items(&self) {
        let number_of_lines = self.get_number_of_lines();
        let last_index = self.get_last_index();

        let mut lines = self.lines.lock_mut();
        for i in 0..number_of_lines {
            lines.push_cloned(Line::new(last_index + i));
        }

        self.set_last_index(last_index + number_of_lines);
    }

    pub fn remove_item(&self, original_index: usize) {
        let real_index = self.lines.lock_ref().iter().position(|line| line.index == original_index);

        if let Some(real_index) = real_index {
            self.lines.lock_mut().remove(real_index);
        }

    }
}