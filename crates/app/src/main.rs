use dioxus::prelude::*;
use log::LevelFilter;

use crate::components::steam_info::stream_info;

mod components;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let future = use_future(cx, (), |_| async move {
        browser_apis::query(serde_json::json!({ "active": true, "lastFocusedWindow": true }))
            .await
            .unwrap()
    });
    cx.render(rsx!(
        div { class:"text-gray-400 bg-gray-900 body-font",
            // style: "text-align: center;",
            // stream_info {account: "test name"}
            div { "{future.value():?}" }

            h1 { "streams" }

            ul {
                li {
                    stream_info {account: "test name"}
                }
                li { class: "pb-3 sm:pb-4",
                    stream_info {account: "fart"}
                }
            }
        }
    ))
}
