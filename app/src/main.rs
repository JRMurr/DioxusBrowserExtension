use dioxus::prelude::*;
use log::{info, LevelFilter};

mod utils;

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
        // use web_extensions::tabs;
        // let query = tabs::QueryDetails {
        //     active: None,
        //     audible: None,
        //     auto_discardable: None,
        //     cookie_store_id: None,
        //     current_window: None,
        //     discarded: None,
        //     hidden: None,
        //     highlighted: None,
        //     index: None,
        //     muted: None,
        //     last_focused_window: None,
        //     pinned: None,
        //     status: None,
        //     title: None,
        //     url: None,
        //     window_id: 0, // TODO: this should be optional
        //     window_type: None,
        // };

        // let res = tabs::query(&query).await;
        // let props = tabs::CreateProperties {
        //     active: false,
        //     url: "https://google.com/",
        // };
        // let res = tabs::create(props).await;

        let res =
            utils::query(serde_json::json!({ "active": true, "lastFocusedWindow": true })).await;
        info!("got: {:?}", res);
    });
    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { "🌗 Dioxus 🚀" }
            h3 { "Frontend that scales22222." }
            p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
        }
    ))
}
