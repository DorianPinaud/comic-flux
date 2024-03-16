#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::icon::Icon;

pub fn Menu() -> Element {
    rsx!(div {
        class: "menu",
        button {
            Icon {
                icon: "\u{f56f}".to_owned()
            }
        }
    })
}