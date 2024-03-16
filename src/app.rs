#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::header::Header;

pub fn App() -> Element {
    rsx!(div {
        class: "app",
        Header {}
    })
}