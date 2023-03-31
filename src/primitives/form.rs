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
            })
        )
    })
}