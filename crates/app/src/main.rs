use dioxus::prelude::*;
use log::{info, LevelFilter};

use crate::components::steam_info::stream_info;

mod components;

fn main() {
    // init debug tool for WebAssembly
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    info!("hi");
    let future = use_future(cx, (), |_| async move {
        browser_apis::query(serde_json::json!({ "active": true, "lastFocusedWindow": true }))
            .await
            .unwrap()
    });
    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { "🌗 Dioxus 🚀" }
            h3 { "Frontend that slapszzz." }
            p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
            stream_info {account: "test name"}
            div { "{future.value():?}" }
        }
    ))
}
