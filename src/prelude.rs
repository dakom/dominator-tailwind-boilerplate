pub use anyhow::Result;
use dominator::DomBuilder;
pub use dominator::{clone, events, html, svg, with_node, Dom};
pub use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
pub use serde::{Deserialize, Serialize};
pub use std::cell::RefCell;
pub use std::rc::Rc;
pub use wasm_bindgen::prelude::*;

pub type MixinStub<T> = fn(DomBuilder<T>) -> DomBuilder<T>;

pub use crate::{
    config::*,
};