use crate::{prelude::*, components::button::ButtonColor};
use futures_signals::signal::{from_future, option};
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
};

pub struct ButtonPage {
    pub last_clicked: Mutable<Option<ButtonColor>>,
}

impl ButtonPage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            last_clicked: Mutable::new(None),
        })
    }
}
