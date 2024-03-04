#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::menu::Menu;
use crate::components::icon::Icon;

#[component]
pub fn Header(cx: Scope) -> Element {
    let menu_opened = use_state(cx, bool::default);
    render!(div {
        class: "header",
        div {
            h1 {
               "Comic Flux"
            }
            button {
                onclick: |_| menu_opened.set(true),
                Icon {
                    icon: "\u{f0c9}".to_owned()
                }
            }
        }
        if **menu_opened 
        {
            rsx!(
                div {
                    Menu {}
                    button {
                        onclick: |_| menu_opened.set(false),
                        Icon {
                            icon: "\u{f00d}".to_owned()
                        }
                    }
                }
            )
        }
    })
}
