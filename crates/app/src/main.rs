use dioxus::prelude::*;
use log::LevelFilter;

use crate::{
    components::{
        selected_stream::selected_stream, steam_info::stream_info, stream_list::stream_list,
    },
    twitch::get_twitch_tabs,
};

mod components;
mod twitch;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let future = use_future(cx, (), |_| async move { get_twitch_tabs().await });
    log::info!("fut: {:#?}", future.value());

    let rendered_stream_list = match future.value() {
        Some(Ok(stream_tabs)) => rsx!(stream_list {
            stream_tabs: stream_tabs.clone(), // TODO: no clone
        }),
        Some(_err) => rsx!("Error getting stream tabs"), // TODO: good error handling..
        None => rsx!("Retrieving stream tabs"),
    };

    cx.render(rsx!(
        div { class:"text-gray-400 bg-gray-900 body-font",

            h1 { "streams" }

            rendered_stream_list

        }
    ))
}
