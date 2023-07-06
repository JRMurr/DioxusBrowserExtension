use dioxus::prelude::*;

use crate::{components::steam_info::stream_info, twitch::StreamTab};

#[derive(Props, PartialEq)]
pub struct TabListProps {
    stream_tabs: Vec<StreamTab>,
}

pub fn tab_list<'a>(cx: Scope<'a, TabListProps>) -> Element {
    let selected_streams = use_state::<Vec<StreamTab>>(cx, || cx.props.stream_tabs.clone());

    // cx.spawn(async {
    //     if let Ok(tabs) = get_twitch_tabs().await {
    //         selected_streams.set(tabs)
    //     }
    // });

    // use_future(cx, (), |_| async move {
    //     // match get_twitch_tabs().await {
    //     //     Ok(tabs) => selected_streams.set(tabs),
    //     //     Err(_) => todo!(),
    //     // }

    //     if let Ok(tabs) = get_twitch_tabs().await {
    //         selected_streams.set(tabs)
    //     }
    // });

    let rendered_tabs = selected_streams.iter().map(|tab| {
        rsx!(li { key: "{tab.id}",
            stream_info {account: "{tab.channel}"}
        })
    });

    cx.render(rsx!(ul {
        rendered_tabs /* li {
                       *     stream_info {account: "test name"}
                       * }
                       * li {
                       *     stream_info {account: "fart"}
                       * } */
    }))
}
