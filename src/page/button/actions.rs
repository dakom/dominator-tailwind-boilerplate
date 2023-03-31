use crate::{prelude::*, components::button::ButtonColor};
use super::state::*;

impl ButtonPage {
    pub fn handle_click(&self, color: ButtonColor) {
        self.last_clicked.set_neq(Some(color));
    }
}