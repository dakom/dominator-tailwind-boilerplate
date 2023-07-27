use dominator::{apply_methods, EventOptions};

use super::state::*;
use crate::{prelude::*, components::{header::Header, button::{Button, ButtonColor}}};

impl ListPage {
    pub fn render(self: Arc<Self>) -> Dom {
        let state = self;

        html!("div", {
            .child(Header::new().render())

            .child(html!("div", {
                .class(["flex", "flex-col", "gap-10", "w-full", "justify-center", "items-center", "pt-10"])
                .child(Button::new(ButtonColor::Green).render_click_mixin(
                    clone!(state => move || state.add_items()),
                    |dom| {
                        apply_methods!(dom, {
                            .class("text-4xl")
                            .text("Add Lines")
                        })
                    }
                ))
                .child(state.clone().render_number_input())
            }))

            .child(html!("div", {
                .class(["w-full", "pt-10"])
                .children_signal_vec(state.lines.signal_vec_cloned().map(clone!(state => move |line| {
                    line.render(state.clone())
                })))
            }))

        })
    }


    fn render_number_input(self: Arc<Self>) -> Dom {
        let state = self;
        html!("div", {
            .class(["flex", "gap-5"])
            .child(html!("label", {
                .class(["flex", "flex-1"])
                .text("Number of lines:")
            }))
            .child(html!("input", {
                .attr("type", "number")
                .attr("value", &state.get_number_of_lines().to_string())
                .class(["block","border","border-gray-300","focus:outline-none","focus:shadow-outline-blue","focus:border-blue-300"])
                .event(clone!(state => move |evt:events::Input| {
                    state.set_number_of_lines(evt.value().unwrap_or_default().parse::<usize>().unwrap_or(0));
                }))
            }))
        })
    }

}

impl Line {
    pub fn render(&self, parent: Arc<ListPage>) -> Dom {
        let state = self;
        html!("div", {
            .class(["rounded-t-lg","border","border-neutral-200","bg-white"])
            .children([
                html!("div", {
                    .class(["text-lg", "mb-0"])
                    .child(
                        html!("button", {
                            .class(["group","relative","flex","w-full","items-center","rounded-t-[15px]","border-0","bg-white","py-4","px-5","text-left","text-base","text-neutral-800","transition","[overflow-anchor:none]","hover:z-[2]","focus:z-[3]","focus:outline-none","[&:not([data-te-collapse-collapsed])]:bg-white","[&:not([data-te-collapse-collapsed])]:text-primary","[&:not([data-te-collapse-collapsed])]:[box-shadow:inset_0_-1px_0_rgba(229,231,235)]"])
                            .child(html!("div", {
                                .class(["flex", "w-full", "justify-between", "items-center"])
                                .child(html!("div", {
                                    .text(&state.text())
                                }))
                                .child(html!("div", {
                                    .class(["flex", "gap-5"])
                                    .child(state.clone().render_trash(parent.clone()))
                                    .child(state.render_chevron())
                                }))
                            }))
                            .event(clone!(state => move |evt: events::Click| {
                                let name = evt
                                    .target()
                                    .and_then(|target| {
                                        js_sys::Reflect::get(&target, &"data-name".into())
                                            .ok()
                                            .and_then(|x| x.as_string())
                                    });
                                if name.unwrap_or_default() != "trash" {
                                    state.expanded.set_neq(!state.expanded.get_cloned());
                                }
                            }))
                        })
                    )
                    .child_signal(state.expanded.signal().map(clone!(state => move |expanded| {
                        if expanded {
                            Some(state.render_description())
                        } else {
                            None
                        }
                    })))
                }),
            ])
        })
    }
    
    fn render_description(&self) -> Dom {
        html!("div", {
            .class(["pl-4", "pr-4", "pb-4"])
            .child(html!("div", {
                .class(["p-4"])
                .text(&self.description())
            }))
        })
    }

    fn render_trash(&self, parent: Arc<ListPage>) -> Dom {
        let state = self;
        let original_index = state.index;

        html!("img", {
            .class(["cursor-pointer"])
            .prop("data-name", "trash")
            .attr("src", &CONFIG.media_url("trash.svg"))
            .attr("width", "16") 
            .attr("height", "16") 
            .event(clone!(parent, original_index => move |evt: events::Click| {
                parent.remove_item(original_index);
            }))
        })
    }

    fn render_chevron(&self) -> Dom {
        html!("div", {
            .class(["ml-auto","h-5","w-5","fill-[#336dec]","transition-transform","duration-200","ease-in-out","motion-reduce:transition-none"])
            .class_signal("rotate-180", self.expanded.signal().map(|expanded| !expanded))
            .child(
                svg!("svg", {
                    .attr("xmlns", "http://www.w3.org/2000/svg")
                    .attr("fill", "none")
                    .attr("viewBox", "0 0 24 24")
                    .attr("stroke-width", "1.5")
                    .attr("stroke", "currentColor")
                    .class(["h-6","w-6"])
                    .child(
                        svg!("path", {
                            .attr("stroke-linecap", "round")
                            .attr("stroke-linejoin", "round")
                            .attr("d", "M19.5 8.25l-7.5 7.5-7.5-7.5")
                        })
                    )
                })
            )
        })
    }
}

