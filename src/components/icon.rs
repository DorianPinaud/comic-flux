#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Icon(cx: Scope, icon: String) -> Element {
    render!(span {
        class: "icon",
        "{icon}"
    })
}