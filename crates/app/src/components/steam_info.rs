use dioxus::prelude::*;

#[derive(Props)]
pub struct StreamInfoProps<'a> {
    account: &'a str,
    selected: bool,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn stream_info<'a>(cx: Scope<'a, StreamInfoProps<'a>>) -> Element {
    // TODO: always show an outline
    let background = if cx.props.selected { "bg-blue-500" } else {"bg-black-500"};

    cx.render(
        rsx!(div { 
            button {
                class: "text-white {background} border-0 py-2 px-6 focus:outline-none hover:bg-blue-600 rounded text-lg",
                onclick: move |evt| cx.props.on_click.call(evt),
                "{cx.props.account}"
            }
        }),
    )
}
