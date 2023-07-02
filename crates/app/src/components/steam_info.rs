use dioxus::prelude::*;

#[derive(Props)]
pub struct StreamInfoProps<'a> {
    account: &'a str,
}

pub fn stream_info<'a>(cx: Scope<'a, StreamInfoProps<'a>>) -> Element {
    cx.render(
        rsx!(div { 
            button {
                class: "text-white bg-blue-500 border-0 py-2 px-6 focus:outline-none hover:bg-blue-600 rounded text-lg",
                "fart11: {cx.props.account}"
            }
        }),
    )
}
