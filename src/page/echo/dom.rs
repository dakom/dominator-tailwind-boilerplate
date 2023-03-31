use dominator::EventOptions;

use super::state::*;
use crate::{prelude::*, components::header::Header, primitives::form::{render_text_input, render_submit_button}};

pub(super) const FIELD_CONTENT: &str = "content";

impl EchoPage {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("div", {
            .child(Header::new().render())
            .child(html!("div", {
                .class(["flex", "flex-col", "items-center","justify-center", "w-full", "mt-10"])
                .child(state.clone().render_form())
                .child_signal(state.value.signal_ref(|value| {
                    value.as_ref().map(|value| {
                        html!("div", {
                            .class(["text-4xl", "font-bold", "pt-10"])
                            .text(value)
                        })
                    })
                }))
            }))
        })
    }
    pub fn render_form(self: Rc<Self>) -> Dom {
        let state = self;
        html!("form" => web_sys::HtmlFormElement, {
            .attr("method", "POST")
            .with_node!(elem => {
                .event_with_options(&EventOptions::preventable(), clone!(state => move |evt: events::Submit| {
                    evt.prevent_default();
                    state.clone().handle_form(&elem);
                }))
            })
            .child(html!("div", {
                .class(["mt-10", "flex", "flex-col", "gap-3"])
                .children([
                    render_text_input(FIELD_CONTENT, false, "Content to echo", None),
                ])
            }))
            .children([
                html!("div", {
                    .class("mt-6")
                    .child(render_submit_button("Submit"))
                }),
            ])
        })
    }
}