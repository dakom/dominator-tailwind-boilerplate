use crate::prelude::*;
use futures_signals::signal::{from_future, option};
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
};

pub struct EchoPage {
    pub value: Mutable<Option<String>>
}

impl EchoPage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            value: Mutable::new(None)
        })
    }
}
