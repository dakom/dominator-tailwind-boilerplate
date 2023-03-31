use crate::prelude::*;

pub struct Button {
    pub color: ButtonColor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonColor {
    Yellow,
    Rose,
    Green,
    Blue,
}

impl Button {
    pub fn new(color: ButtonColor) -> Rc<Self> {
        Rc::new(Self {
            color 
        })
    }
}
