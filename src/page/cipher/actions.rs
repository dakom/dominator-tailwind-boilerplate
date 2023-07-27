use ed25519_dalek::{Signature, SigningKey, Signer};
use rand::rngs::OsRng;
use wasm_bindgen::JsCast;
use web_sys::{HtmlFormElement, HtmlInputElement};

use super::state::*;

impl CipherPage {
    pub fn generate_signing_key(&self) {
        let state = self;
        state.error.set(None);
        state.signature.set(None);

        let mut csprng = OsRng{};
        let keypair = SigningKey::generate(&mut csprng);
        state.signing_key.set(Some(keypair));
    }

    pub fn sign_message(&self, elem: &web_sys::HtmlFormElement) {
        let state = self;
        state.error.set(None);

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

        match get_string(FIELD_MESSAGE) {
            None => {
                state.error.set(Some("Message cannot be empty".to_string()));
            },
            Some(content) => {
                match state.signing_key.get_cloned() {
                    Some(signing_key) => {
                        state.signature.set(Some(signing_key.sign(content.as_bytes())));
                    },
                    None => {
                        state.error.set(Some("No signing key".to_string()));
                    },
                }
            }
        }
    }
}