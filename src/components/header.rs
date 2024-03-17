#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::menu::Menu;
use crate::components::icon::Icon;

pub fn Header() -> Element {
    let mut menu_opened = use_signal(bool::default);
    rsx! {div {
        class: "header",
        div {
            h1 {
               "Comic Flux"
            }
            button {
                onclick: move |_| {
                    let val = *menu_opened.read(); 
                    menu_opened.set(!val);
                },
                Icon {
                    icon: "\u{f0c9}".to_owned()
                }
            }
        }
    }
    if *menu_opened.read() 
    {
        Menu {}
    }
}
}
