use dioxus::prelude::*;

#[derive(Props)]
pub struct SelectedStreamProps<'a> {
    account: &'a str,
}

pub fn selected_stream<'a>(cx: Scope<'a, SelectedStreamProps<'a>>) -> Element {
    cx.render(rsx!(div {
        button {
            class: "text-white bg-zinc-800 border-0 py-2 px-6 focus:outline-none  rounded text-lg",
            "eeee: {cx.props.account}"
            cross_icon {}
        }
    }))
}

fn cross_icon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg { class: "flex-no-shrink fill-current",
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            // // class: " text-white p-2 bg-indigo-500 rounded-full",
            view_box: "0 0 20 20",
            path { stroke_linecap:"round", stroke_linejoin:"round", stroke_width:"2", d: "M6 18L18 6M6 6l12 12"}
        }
    ))
}
