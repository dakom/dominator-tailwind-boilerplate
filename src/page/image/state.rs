use crate::prelude::*;
use futures_signals::signal::{from_future, option};
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
};

pub struct ImagePage {
}

impl ImagePage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
        })
    }
}
