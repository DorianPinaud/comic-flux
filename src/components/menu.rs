#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::icon::Icon;

#[component]
pub fn Menu(cx: Scope) -> Element {
    render!(div {
        class: "menu",
        button {
            Icon {
                icon: "\u{f56f}".to_owned()
            }
        }
    })
}