use crate::prelude::*;
use futures_signals::signal::{from_future, option};
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
};

#[derive(Clone)]
pub struct ImagePage {
}

impl ImagePage {
    pub fn new() -> Self {
        Self {
        }
    }
}
