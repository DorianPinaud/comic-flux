#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus::desktop::Config;

mod app;
mod components;
mod models;

use crate::app::App;

fn main() {
    let index_content = include_str!("index.html").to_string();
    LaunchBuilder::desktop().with_cfg(Config::new().with_custom_index(index_content)).launch(App);
}
