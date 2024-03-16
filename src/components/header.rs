#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::menu::Menu;
use crate::components::icon::Icon;

#[component]
pub fn Header() -> Element {
    let mut menu_opened = use_signal(bool::default);
    rsx! {div {
        class: "header",
        div {
            h1 {
               "Comic Flux"
            }
            button {
                onclick: move |_| *menu_opened.write() = true,
                Icon {
                    icon: "\u{f0c9}".to_owned()
                }
            }
        }
        if *menu_opened.read() 
        {
                div {
                    Menu {},
                    button {
                        onclick: move |_| *menu_opened.write() = false,
                        Icon {
                            icon: "\u{f00d}".to_owned()
                        }
                    }
                }
            }
    }}
}
