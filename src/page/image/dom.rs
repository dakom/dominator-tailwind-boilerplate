use super::state::*;
use crate::{prelude::*, components::header::Header};

impl ImagePage {
    pub fn render(&self) -> Dom {
        html!("div", {
            .child(Header::new().render())

            .child(html!("div", {
                .class(["flex", "w-full", "text-center", "justify-center", "items-center", "pt-10", "text-2xl"])
                .child(html!("img", {
                    .attr("src", &CONFIG.media_url("star.svg"))
                }))
            }))
        })
    }
}
