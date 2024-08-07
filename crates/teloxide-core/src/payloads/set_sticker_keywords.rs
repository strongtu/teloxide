//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::True;

impl_payload! {
    /// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetStickerKeywords (SetStickerKeywordsSetters) => True {
        required {
            /// File identifier of the sticker
            pub sticker: String [into],
        }
        optional {
            /// A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters
            pub keywords: Vec<String> [collect],
        }
    }
}
