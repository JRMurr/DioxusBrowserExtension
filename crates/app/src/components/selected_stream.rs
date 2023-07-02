use dioxus::prelude::*;

#[derive(Props)]
pub struct SelectedStreamProps<'a> {
    account: &'a str,
}

pub fn selected_stream<'a>(cx: Scope<'a, SelectedStreamProps<'a>>) -> Element {
    // TODO: make test left side, x on right side
    cx.render(rsx!(div {
        button {
            class: "inline-flex items-center text-white bg-zinc-800 border-0 py-2 px-6 focus:outline-none rounded text-lg",
            "{cx.props.account}" cross_icon {}
        }
    }))
}

fn cross_icon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 20 20",
            width: "5%",
            height: "auto",
            path { stroke_linecap:"round", stroke_linejoin:"round", stroke_width:"2", d: "M6 18L18 6M6 6l12 12"}
        }
    ))
}
