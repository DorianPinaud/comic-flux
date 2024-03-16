#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Icon( icon: String) -> Element {
    rsx!(span {
        class: "icon",
        "{icon}"
    })
}