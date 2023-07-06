use gloo_utils::format::JsValueSerdeExt;
use js_sys::Object;
use miette::{IntoDiagnostic, Result};
use wasm_bindgen::prelude::*;
pub mod tab;

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
