use crate::{prelude::*, components::button::ButtonColor};
use futures_signals::signal::{from_future, option};
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
};

#[derive(Clone)]
pub struct ButtonPage {
    pub last_clicked: Mutable<Option<ButtonColor>>,
}

impl ButtonPage {
    pub fn new() -> Self {
        Self {
            last_clicked: Mutable::new(None),
        }
    }
}
