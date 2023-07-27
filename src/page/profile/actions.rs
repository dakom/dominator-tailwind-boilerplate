use super::{state::*, FIELD_ID};
use crate::{prelude::*, route::Route};
use web_sys::{HtmlFormElement, HtmlInputElement};

impl ProfilePage {
    pub fn handle_form(&self, elem: &HtmlFormElement) {
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
        if let Some(content) = get_string(FIELD_ID) {
            Route::Profile(Some(content)).go_to_url();
        }
    }
}