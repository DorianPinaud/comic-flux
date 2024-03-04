#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::header::Header;

#[component]
pub fn App(cx: Scope) -> Element {
    render!(div {
        class: "app",
        Header {}
    })
}