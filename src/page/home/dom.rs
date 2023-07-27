use super::state::*;
use crate::{prelude::*, components::header::Header, route::Route};

impl HomePage {
    pub fn render(&self) -> Dom {
        let state = self;

        html!("div", {
            .child(Header::new().render())
            .child(html!("div", {
                .class(["flex", "flex-col", "w-full", "text-center", "justify-center", "items-center", "pt-10", "text-2xl"])
                .child(html!("div", {
                    .text("Welcome to the Dominator/Tailwind starter template.")
                }))
            }))
            .child(html!("div", {
                .class("ml-10")
                .children([
                    ("Image", Route::Image, "Loads an image from configured media url"),
                    ("Echo", Route::Echo, "Simple example of using a Mutable"),
                    ("Profile", Route::Profile(None), "Push state / permalink urls"),
                    ("Button", Route::Button, "Abstract common styles into a component"),
                    ("List", Route::List, "More advanced usage of Mutable/MutableVec")
                    ].iter().map(|(name, route, description)| {
                        html!("div", {
                            .class(["pt-10", "hover:text-indigo-800"])
                            .child(html!("a", {
                                .attr("href", &route.href())
                                .class(["text-2xl", "pt-10"])
                                .child(html!("div", {
                                    .class("font-bold")
                                    .text(name)
                                }))
                                .child(html!("div", {
                                    .text(description)
                                })) 
                            }))
                        })
                    })
                )
            }))
        })
    }
}
