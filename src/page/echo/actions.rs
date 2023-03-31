use super::{state::*, FIELD_CONTENT};
use crate::{prelude::*, route::Route};
use web_sys::{HtmlFormElement, HtmlInputElement};

impl EchoPage {
    pub fn handle_form(self: Rc<Self>, elem: &HtmlFormElement) {
        let state = self;
        let get_string = |name: &str| -> Option<String> {
            let value: String = elem
                .get_with_name(name)?
                .unchecked_into::<HtmlInputElement>()
                .value();
            if value.is_empty() {
                None
            } else {
                Some(value)
            }
        };
        if let Some(content) = get_string(FIELD_CONTENT) {
            if content.is_empty() {
                state.value.set_neq(None);
            } else {
                state.value.set_neq(Some(content));
            }
        }
    }
}