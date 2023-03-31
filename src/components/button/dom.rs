use super::state::*;
use crate::prelude::*;
use dominator::DomBuilder;
use web_sys::HtmlElement;

impl Button {
    pub fn render_click(&self, on_click: impl Fn() + 'static) -> Dom {
        Self::render_mixin(self, |dom: DomBuilder<HtmlElement>| {
            dom.event(move |_evt: events::Click| {
                on_click();
            })
        })
    }

    pub fn render_click_mixin<F>(&self, on_click: impl Fn() + 'static, mixin: F) -> Dom 
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static,
    {
        Self::render_mixin(self, |dom: DomBuilder<HtmlElement>| {
            dom
                .apply(mixin)
                .event(move |_evt: events::Click| {
                    on_click();
                })
        })
    }

    pub fn render_mixin<F>(&self, mixin: F) -> Dom
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static,
    {
        Self::render_maybe_mixin_inner(self, Some(mixin))
    }

    fn render_maybe_mixin_inner<F>(&self, mixin: Option<F>) -> Dom
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static,
    {

        html!("button", {
            .attr("type", "button")
            .apply(|dom| self.color.mixin_color_class(dom))
            .apply_if(mixin.is_some(), |dom| dom.apply(mixin.unwrap()))
        })
    }

}

impl ButtonColor {
    pub fn mixin_color_class(&self, dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        match self {
            Self::Yellow => dom.class([
                "bg-custom-yellow-200",
                "hover:bg-custom-yellow-100",
                "text-rose-600",
            ]),
            Self::Rose => dom.class([
                "bg-rose-500",
                "hover:bg-rose-600",
                "text-white",
            ]),
            Self::Green => dom.class([
                "bg-emerald-500",
                "hover:bg-emerald-600",
                "text-white",
            ]),
            Self::Red => dom.class([
                "bg-red-500", 
                "hover:bg-red-600", 
                "text-white",
            ]),
        }
        .class([
            "relative",
            "inline-flex",
            "items-center",
            "outline-0",
            "px-4",
            "py-2",
            "border",
            "border-transparent",
            "shadow-sm",
            "text-sm",
            "font-medium",
            "rounded-md",
        ])
    }
}
