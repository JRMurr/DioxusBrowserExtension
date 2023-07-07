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

    let rendered_tabs = selectable_streams.iter().enumerate().map(
        |(idx, SelectableStream { stream, selected })| {
            rsx!(li { key: "{stream.id}",
                stream_info {account: "{stream.channel}", selected: *selected,
                on_click: move |_event| selectable_streams.modify(|streams| {
                    // TODO: switch to use ref https://dioxuslabs.com/docs/0.3/guide/en/interactivity/hooks.html#use_ref-hook
                    let mut new_streams = streams.to_owned();
                    new_streams[idx].selected = !new_streams[idx].selected;
                    new_streams
                })}
            })
        },
    );

    let rendered_selected_stream = selectable_streams // TODO: if none selected put dummy text there
        .iter()
        .enumerate()
        .filter(|(_idx, v)| v.selected)
        .map(|(_idx, SelectableStream { stream, .. })| {
            rsx!(li { key: "{stream.id}",
                selected_stream {account: "{stream.channel}"}
            })
        });

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
