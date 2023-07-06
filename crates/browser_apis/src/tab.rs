use std::collections::HashMap;

use miette::Result;
use serde::{Deserialize, Serialize};
use url::Url;
use web_extensions_sys as sys;

use crate::{js_from_serde, object_from_js, serde_from_js_result};

// taken from https://github.com/web-extensions-rs/web-extensions/blob/main/src/tabs/mod.rs

/// The ID of the tab.
///
/// Tab IDs are unique within a browser session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TabId(i32);

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#type-Tab>
#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    /// https://developer.chrome.com/docs/extensions/reference/tabs/#property-Tab-id
    pub id: Option<TabId>,
    /// https://developer.chrome.com/docs/extensions/reference/tabs/#property-Tab-url
    pub url: Option<Url>,

    #[serde(flatten)]
    pub unknown_fields: HashMap<String, serde_json::Value>,
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#method-query>
pub async fn query<T: serde::Serialize>(details: T) -> Result<Vec<Tab>> {
    let js_details = js_from_serde(&details)?;
    let result = sys::chrome()
        .tabs()
        .query(object_from_js(&js_details)?)
        .await;
    serde_from_js_result(result)
}
