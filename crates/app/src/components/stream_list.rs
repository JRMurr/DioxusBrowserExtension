use dioxus::prelude::*;

use crate::{
    components::{selected_stream::selected_stream, steam_info::stream_info},
    twitch::StreamTab,
};

#[derive(Props, PartialEq)]
pub struct StreamListProps {
    stream_tabs: Vec<StreamTab>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SelectableStream {
    stream: StreamTab,
    selected: bool,
}

impl From<StreamTab> for SelectableStream {
    fn from(stream: StreamTab) -> Self {
        Self {
            stream,
            selected: true,
        }
    }
}

pub fn stream_list(cx: Scope<'_, StreamListProps>) -> Element {
    let selectable_streams = use_state::<Vec<SelectableStream>>(cx, || {
        cx.props
            .stream_tabs
            .iter()
            .map(|v| v.clone().into())
            .collect()
    });

    let rendered_tabs = selectable_streams
        .iter()
        .map(|SelectableStream { stream, selected }| {
            rsx!(li { key: "{stream.id}",
                stream_info {account: "{stream.channel}"}
            })
        });

    let rendered_selected_stream = selectable_streams.iter().filter(|v| v.selected).map(
        |SelectableStream { stream, selected }| {
            rsx!(li { key: "{stream.id}",
                selected_stream {account: "{stream.channel}"}
            })
        },
    );

    cx.render(rsx!(div { class: "grid grid-cols-3 gap-4",

        div {class: "col-span-1",
            ul {
                rendered_tabs
            }
        }

        ul { class: "col-span-1 col-start-3",
            rendered_selected_stream
        }
    }))
}
