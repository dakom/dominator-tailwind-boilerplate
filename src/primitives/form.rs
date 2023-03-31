use crate::prelude::*;

pub fn render_text_input(name: &str, required: bool, label: &str, value: Option<&str>) -> Dom {
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
            .class(["block","border","border-gray-300","focus:outline-none","focus:shadow-outline-blue","focus:border-blue-300"])
        }))
    })
}

pub fn render_submit_button(label: &str) -> Dom {
    html!("button", {
        .attr("type", "submit")
        .class(["group","relative","w-full","flex","justify-center","py-2","px-4","border","border-transparent","text-sm","leading-5","font-medium","rounded-md","text-white","bg-indigo-600","hover:bg-indigo-500","focus:outline-none","focus:border-indigo-700","focus:shadow-outline-indigo","active:bg-indigo-700","transition","duration-150","ease-in-out"])
        .text(label)
        .child(
            html!("span", {
                .class(["absolute","left-0","inset-y-0","flex","items-center","pl-3"])
                .child(
                    svg!("svg", {
                        .class(["h-5","w-5","text-indigo-500","group-hover:text-indigo-400","transition","ease-in-out","duration-150"])
                        .attr("fill", "currentColor")
                        .attr("viewBox", "0 0 20 20")
                        .child(
                            svg!("path", {
                                .attr("fill-rule", "evenodd")
                                .attr("d", "M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z")
                                .attr("clip-rule", "evenodd")
                            })
                        )
                    })
                )
            })
        )
    })
}