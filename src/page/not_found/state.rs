use crate::prelude::*;
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
};

#[derive(Clone)]
pub struct NotFoundPage {}

impl NotFoundPage {
    pub fn new() -> Self {
        Self {}
    }
}
