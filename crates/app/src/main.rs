use dioxus::prelude::*;
use log::LevelFilter;

use crate::components::{selected_stream::selected_stream, steam_info::stream_info};

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

            h1 { "streams" }

            div { class: "grid grid-cols-3 gap-4",
                ul { class: "col-span-1",
                    li {
                        stream_info {account: "test name"}
                    }
                    li {
                        stream_info {account: "fart"}
                    }
                }

                ul { class: "col-span-1 col-start-3",
                    li {
                        selected_stream {account: "test name"}
                    }
                    li {
                        selected_stream {account: "fart"}
                    }
                }
            }

        }
    ))
}
