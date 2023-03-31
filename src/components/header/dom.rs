use dominator::apply_methods;

use super::state::*;
use crate::{prelude::*, components::button::*, route::Route};

impl Header {
    pub fn render(self: Rc<Self>) -> Dom {
        html!("div", {
            .class(["flex", "justify-between", "items-center", "p-4", "bg-custom-green", "border-b", "border-custom-green-dark", "text-white"])
            .child(self.clone().render_left())
            .child(self.clone().render_right())
        })
    }

    pub fn render_left(self: Rc<Self>) -> Dom {
        log::info!("{}", Route::Home.href());
        html!("div", {
            .child(html!("a", {
                .class(["text-xl", "font-bold"])
                .text("Welcome")
                .attr("href", &Route::Home.href()) 
            }))
        })
    }
    pub fn render_right(self: Rc<Self>) -> Dom {
        html!("div", {
            .class(["flex", "items-center", "gap-4"])
            .children(vec![
                    ("Image", Route::Image),
                    ("Echo", Route::Echo),
                    ("Profile", Route::Profile(None)),
                ]
                .into_iter()
                .map(|(text, route)| {
                    html!("a", {
                        .class(["text-xl", "font-bold"])
                        .class_signal("underline", Route::current_signal().map(
                            clone!(route => move |current_route| {
                                match route {
                                    Route::Profile(_) => std::mem::discriminant(&current_route) == std::mem::discriminant(&Route::Profile(None)),
                                    _ => route == current_route
                                }
                            }) 
                        ))
                        .text(text)
                        .attr("href", &route.href())
                    })
                })
            )
        })
    }

    // pub fn render_right_auth(&self) -> Dom {
    //     html!("div", {
    //         .class(["flex", "items-center", "gap-4"])
    //         .children([
    //             html!("a", {
    //                 .child(Button::new(ButtonColor::Yellow).render_mixin(|dom| apply_methods!(dom, {
    //                     .text("Echo")
    //                 })))
    //                 .attr("href", &Route::Echo.href())
    //             }),
    //         ])
    //     })
    // }

}