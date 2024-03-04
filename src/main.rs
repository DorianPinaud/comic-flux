#![allow(non_snake_case)]

mod app;
mod components;

use crate::app::App;

fn main() {
    let index_content = include_str!("index.html").to_string();
    dioxus_desktop::launch_cfg(App, dioxus_desktop::Config::new().with_custom_index(index_content));
}
