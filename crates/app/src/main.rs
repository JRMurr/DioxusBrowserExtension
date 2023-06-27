use dioxus::prelude::*;
use log::LevelFilter;

use crate::components::steam_info::stream_info;

mod components;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let future = use_future(cx, (), |_| async move {
        browser_apis::query(serde_json::json!({ "active": true, "lastFocusedWindow": true }))
            .await
            .unwrap()
    });
    // cx.render(rsx!(
    //     // div {
    //     //     style: "text-align: center;",
    //     //     stream_info {account: "test name"}
    //     //     div { "{future.value():?}" }
    //     // }
    // ))

    cx.render(rsx!(
        div {
            header { class: "text-gray-400 bg-gray-900 body-font",
                div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
                    a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
                        span { class: "ml-3 text-xl", "Hello Dioxus!"}
                    }
                    nav { class: "md:ml-auto flex flex-wrap items-center text-base justify-center",
                        a { class: "mr-5 hover:text-white", "First Link"}
                        a { class: "mr-5 hover:text-white", "Second Link"}
                        a { class: "mr-5 hover:text-white", "Third Link"}
                        a { class: "mr-5 hover:text-white", "Fourth Link"}
                    }
                    button {
                        class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
                        "Button"
                    }
                }
            }

            section { class: "text-gray-400 bg-gray-900 body-font",
                div { class: "container mx-auto flex px-5 py-24 md:flex-row flex-col items-center",
                    div { class: "lg:flex-grow md:w-1/2 lg:pr-24 md:pr-16 flex flex-col md:items-start md:text-left mb-16 md:mb-0 items-center text-center",
                        h1 { class: "title-font sm:text-4xl text-3xl mb-4 font-medium text-white",
                            br { class: "hidden lg:inline-block" }
                            "Dioxus Sneak Peek"
                        }
                        p {
                            class: "mb-8 leading-relaxed",

                            "Dioxus is a new UI framework that makes it easy and simple to write cross-platform apps using web
                            technologies! It is functional, fast, and portable. Dioxus can run on the web, on the desktop, and
                            on mobile and embedded platforms."

                        }
                        div { class: "flex justify-center",
                            button {
                                class: "inline-flex text-white bg-indigo-500 border-0 py-2 px-6 focus:outline-none hover:bg-indigo-600 rounded text-lg",
                                "Learn more"
                            }
                            button {
                                class: "ml-4 inline-flex text-gray-400 bg-gray-800 border-0 py-2 px-6 focus:outline-none hover:bg-gray-700 hover:text-white rounded text-lg",
                                "Build an app"
                            }
                        }
                    }
                    div { class: "lg:max-w-lg lg:w-full md:w-1/2 w-5/6",
                        img {
                            class: "object-cover object-center rounded",
                            src: "https://i.imgur.com/oK6BLtw.png",
                            referrerpolicy:"no-referrer",
                            alt: "hero",
                        }
                    }
                }
            }
        }
    ))
}
