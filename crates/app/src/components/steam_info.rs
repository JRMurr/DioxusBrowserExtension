use dioxus::prelude::*;

#[derive(Props)]
pub struct StreamInfoProps<'a> {
    account: &'a str,
}

pub fn stream_info<'a>(cx: Scope<'a, StreamInfoProps<'a>>) -> Element {
    cx.render(rsx!(div {
        "account: {cx.props.account}"
    }))
}
