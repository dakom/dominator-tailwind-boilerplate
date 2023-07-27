use ed25519_dalek::{SigningKey, Signature};

use crate::prelude::*;

pub(super) const FIELD_MESSAGE: &str = "message";

pub struct CipherPage {
    pub signing_key: Mutable<Option<SigningKey>>,
    pub error: Mutable<Option<String>>,
    pub signature: Mutable<Option<Signature>>,
}

impl CipherPage {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            signing_key: Mutable::new(None),
            error: Mutable::new(None),
            signature: Mutable::new(None),
        })
    }
}