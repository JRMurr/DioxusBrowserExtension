use gloo_utils::format::JsValueSerdeExt;
use js_sys::Object;
use miette::{IntoDiagnostic, Result};
use wasm_bindgen::prelude::*;
use web_extensions_sys as sys;

// stolen from https://github.com/web-extensions-rs/web-extensions/blob/main/src/util.rs

pub(crate) fn js_from_serde<T: serde::Serialize>(v: &T) -> Result<JsValue> {
    JsValue::from_serde(v).into_diagnostic()
}

pub(crate) fn object_from_js(v: &JsValue) -> Result<&Object> {
    if let Some(o) = Object::try_from(v) {
        Ok(o)
    } else {
        miette::bail!("SAD!")
    }
}

pub(crate) fn serde_from_js_result<T>(v: Result<JsValue, JsValue>) -> Result<T>
where
    T: for<'a> serde::Deserialize<'a>,
{
    match v {
        Ok(res) => res.into_serde().into_diagnostic(),
        Err(err) => miette::bail!("SAD!: {:?}", err),
    }
}

// pub(crate) fn serde_from_js<T>(v: JsValue) -> Result<T>
// where
//     T: for<'a> serde::Deserialize<'a>,
// {
//     v.into_serde().into_diagnostic()
// }

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#method-query>
pub async fn query(details: serde_json::Value) -> Result<Vec<serde_json::Value>> {
    let js_details = js_from_serde(&details)?;
    let result = sys::chrome()
        .tabs()
        .query(object_from_js(&js_details)?)
        .await;
    serde_from_js_result(result)
}
