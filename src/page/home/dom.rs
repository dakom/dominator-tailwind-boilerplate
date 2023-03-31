use super::state::*;
use crate::{prelude::*, components::header::Header};

impl HomePage {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("div", {
            .child(Header::new().render())
            .child(html!("div", {
                .class(["flex", "flex-col", "w-full", "text-center", "justify-center", "items-center", "pt-10", "text-2xl"])
                .child(html!("div", {
                    .text("Welcome to the Dominator/Tailwind starter template.")
                }))
                .child(html!("div", {
                    .text("Check out the buttons on the top-right")
                }))
            }))
        })
    }
}
