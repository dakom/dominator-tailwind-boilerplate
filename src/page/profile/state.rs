use crate::prelude::*;
use futures_signals::signal::{from_future, option};
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
};

pub struct ProfilePage {
    pub value: Option<String>
}

impl ProfilePage {
    pub fn new(value: Option<String>) -> Rc<Self> {
        Rc::new(Self {
            value
        })
    }
}
