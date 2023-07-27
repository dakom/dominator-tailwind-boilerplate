use crate::{prelude::*, components::button::ButtonColor};
use futures_signals::signal::{from_future, option};
use std::{
    cell::{Cell, RefCell},
    collections::HashSet, sync::atomic::{AtomicUsize, Ordering},
};

pub struct ListPage {
    number_of_lines: AtomicUsize,
    last_index: AtomicUsize,
    pub lines: MutableVec<Line>,
}

impl ListPage {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            number_of_lines: AtomicUsize::new(CONFIG.number_of_list_lines),
            last_index: AtomicUsize::new(0),
            lines: MutableVec::new(),
        })
    }

    pub fn get_number_of_lines(&self) -> usize {
        self.number_of_lines.load(Ordering::SeqCst)
    }

    pub fn set_number_of_lines(&self, value: usize) {
        self.number_of_lines.store(value, Ordering::SeqCst)
    }

    pub fn get_last_index(&self) -> usize {
        self.last_index.load(Ordering::SeqCst)
    }

    pub fn set_last_index(&self, value: usize) {
        self.last_index.store(value, Ordering::SeqCst)
    }
}

#[derive(Clone)]
pub struct Line {
    pub index: usize,
    pub expanded: Mutable<bool>,
}

impl Line {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            expanded: Mutable::new(false),
        }
    }

    pub fn text(&self) -> String {
        format!("Item #{}", self.index + 1)
    }

    pub fn description(&self) -> String {
        format!("Description #{}", self.index + 1)
    }

    pub fn toggle_expanded(&self) {
        self.expanded.set_neq(!self.expanded.get());
    }
}