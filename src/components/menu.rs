#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::user_controls::import_button::ImportButton;

pub fn Menu() -> Element {
    rsx!(div {
        class: "menu",
        ImportButton {}
    })
}