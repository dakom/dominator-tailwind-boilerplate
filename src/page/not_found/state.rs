use crate::prelude::*;
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
};

pub struct NotFoundPage {}

impl NotFoundPage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}
