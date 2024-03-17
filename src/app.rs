#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::header::Header;
use crate::components::page_reader::PageReader;

use crate::models::project::Project;

pub fn App() -> Element {
    use_context_provider(|| Signal::new(Project::default()));
    rsx!(div {
        class: "app",
        Header {}
        PageReader {}
    })
}