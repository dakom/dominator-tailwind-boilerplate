use dominator::apply_methods;

use super::state::*;
use crate::{prelude::*, components::{header::Header, button::{Button, ButtonColor}}};

impl ButtonPage {
    pub fn render(&self) -> Dom {
        let state = self;

        html!("div", {
            .child(Header::new().render())

            .child(html!("div", {
                .class(["flex", "gap-10", "w-full", "justify-center", "items-center", "pt-10"])
                .children(get_colors().into_iter().map(|color| {
                    Button::new(color).render_click_mixin(
                        clone!(state, color => move || state.handle_click(color)),
                        |dom| {
                            apply_methods!(dom, {
                                .class("text-4xl")
                                .text("Click Me!")
                            })
                        }
                    )
                }))
            }))
            .child_signal(state.last_clicked.signal_ref(|value| {
                value.as_ref().map(|value| {
                    html!("div", {
                        .class(["w-full", "text-center", "text-4xl", "font-bold", "pt-10"])
                        .text(&format!("{:?}", value))
                    })
                })
            }))
        })
    }
}

fn get_colors() -> Vec<ButtonColor> {
    vec![ButtonColor::Yellow, ButtonColor::Rose, ButtonColor::Green, ButtonColor::Blue]
}