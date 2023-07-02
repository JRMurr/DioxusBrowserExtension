use dioxus::prelude::*;

#[derive(Props)]
pub struct StreamInfoProps<'a> {
    account: &'a str,
}

pub fn stream_info<'a>(cx: Scope<'a, StreamInfoProps<'a>>) -> Element {
    // cx.render(rsx!(div {
    //     "account: {cx.props.account}"
    // }))

    // cx.render(
    //     rsx!(div { class: "space-y-2 xl:grid xl:grid-cols-4 xl:space-y-0
    // xl:items-baseline",         div { class: "space-y-3 xl:col-span-3",
    //             "account: {cx.props.account}"
    //         }
    //     }),
    // )

    cx.render(
        rsx!(div { //class: "py-8 px-8 max-w-sm mx-auto bg-white rounded-xl shadow-lg space-y-2 sm:py-4 sm:flex sm:items-center sm:space-y-0 sm:space-x-6",
            // div { class: "text-center space-y-2 sm:text-left",
            //     div { class: "space-y-0.5",
            //         p { class: "font-semibold", // text-lg text-black
            //             "{cx.props.account}"
            //         }
            //     }
            // }

            // button { class:"bg-sky-500 hover:bg-sky-700 ", // px-4 py-1 text-sm text-purple-600 font-semibold rounded-full border border-purple-200 hover:text-white hover:bg-purple-600 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-purple-600 focus:ring-offset-2
            // "{cx.props.account}"
            // }
            button {
                class: "text-white bg-indigo-500 border-0 py-2 px-6 focus:outline-none hover:bg-indigo-600 rounded text-lg",
                "fart11: {cx.props.account}"
            }
        }),
    )
}
