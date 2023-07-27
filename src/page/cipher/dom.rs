use dominator::{apply_methods, EventOptions};
use ed25519_dalek::SECRET_KEY_LENGTH;

use super::state::*;
use crate::{
    prelude::*, 
    components::{
        header::Header, 
        button::{Button, ButtonColor}
    },
    primitives::form::{render_text_input, render_submit_button},
};

impl CipherPage {
    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;
        html!("div", {
            .child(Header::new().render())
            .child(html!("div", {
                .class(["flex", "flex-col", "items-center","justify-center", "w-full", "mt-10"])
                .child(
                    Button::new(ButtonColor::Blue).render_click_mixin(
                        clone!(state => move || state.generate_signing_key()),
                        |dom| {
                            apply_methods!(dom, {
                                .class("text-4xl")
                                .text("Generate Signing Key")
                            })
                        }
                    )
                )
                .child_signal(state.signing_key.signal_ref(clone!(state => move |signing_key| {
                    signing_key.as_ref().map(|signing_key| {
                        let signing_key_bytes = signing_key.to_keypair_bytes();
                        let private_key_bytes = &signing_key_bytes[..SECRET_KEY_LENGTH];
                        let public_key_bytes = &signing_key_bytes[SECRET_KEY_LENGTH..];
                        html!("div", {
                            .class(["mt-4"])
                            .child(html!("div", {
                                .child(html!("div", {
                                    .text(&format!("private key: {}", hex::encode(private_key_bytes)))
                                }))
                                .child(html!("div", {
                                    .text(&format!("public key: {}", hex::encode(public_key_bytes)))
                                }))
                            }))
                            .child(state.render_form())
                        })
                    })
                })))
                .child_signal(state.error.signal_ref(clone!(state => move |error| {
                    error.as_ref().map(|error| {
                        html!("div", {
                            .class(["mt-4", "text-red-500"])
                            .text(error)
                        })
                    })
                })))
                .child_signal(state.signature.signal_ref(clone!(state => move |signature| {
                    signature.as_ref().map(|signature| {
                        html!("div", {
                            .class(["flex", "flex-col", "items-center","justify-center", "w-full", "mt-4"])
                            .child(html!("div", {
                                .class("text-center")
                                .text("Signature:")
                            }))
                            .child(html!("div", {
                                .class("text-center")
                                .text(&hex::encode(signature.to_bytes()))
                            }))
                        })
                    })
                })))
            }))
        })
    }

    fn render_form(self: &Arc<Self>) -> Dom {
        let state = self;
        html!("form" => web_sys::HtmlFormElement, {
            .attr("method", "POST")
            .with_node!(elem => {
                .event_with_options(&EventOptions::preventable(), clone!(state => move |evt: events::Submit| {
                    evt.prevent_default();
                    state.clone().sign_message(&elem);
                }))
            })
            .child(html!("div", {
                .class(["mt-10", "flex", "flex-col", "gap-3"])
                .children([
                    render_message(FIELD_MESSAGE, false, "Message", None, clone!(state => move || {
                        state.signature.set(None);
                    })),
                ])
            }))
            .child(render_sign_button("Sign Message"))
        })
    }
}

// should be textarea instead?
pub fn render_message<F: Fn() + Clone + 'static>(name: &str, required: bool, label: &str, value: Option<&str>, on_input: F) -> Dom {
    html!("div", {
        .class(["flex", "w-full", "gap-5"])
        .child(html!("label", {
            .class(["flex", "flex-1"])
            .attr("for", name)
            .text(label)
        }))
        .child(html!("input", {
            .attr("name", name)
            .attr("type", "text")
            .apply_if(required, |dom| {
                dom.attr("required", "")
            })
            .apply_if(value.is_some(), |dom| {
                dom.attr("value", value.unwrap())
            })
            .event(clone!(on_input => move |evt: events::Input| {
                on_input();
            }))
            .class(["block","border","border-gray-300","focus:outline-none","focus:shadow-outline-blue","focus:border-blue-300", "w-full"])
        }))
    })
}

fn render_sign_button(label: &str) -> Dom {
    html!("div", {
        .class(["flex", "flex-col", "items-center","justify-center", "w-full", "mt-10"])
        .child(html!("button", {
            .attr("type", "submit")
            .text(label)
            .apply(|dom| ButtonColor::Blue.mixin_color_class(dom))
            .class(["text-4xl"])
            .child(
                html!("span", {
                    .class(["absolute","left-0","inset-y-0","flex","items-center","pl-3"])
                })
            )
        }))
    })
}