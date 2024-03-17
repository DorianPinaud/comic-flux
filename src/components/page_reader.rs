#![allow(non_snake_case)]
use dioxus::prelude::*;

use base64::prelude::*;

use image::{ codecs::png::PngEncoder, ImageEncoder };

use crate::components::icon::Icon;

use crate::models::project::Project;

pub fn PageReader() -> Element {
    let mut current_img_idx: Signal<usize> = use_signal(|| 0usize);
    let idx = *(current_img_idx.read());
    let project = consume_context::<Signal<Project>>();
    let pages = &(*project.read()).pages;
    let page_count = pages.len();

    if page_count == 0usize {
        None
    } else {
        let current_image = (*pages)[idx].image.clone();
        let width = current_image.width();
        let height = current_image.height();
        let color_type = current_image.color();
        let buffer_source = current_image.into_bytes();
        let mut buffer_target = Vec::<u8>::default();
        let cursor = std::io::Cursor::new(&mut buffer_target);
        let encoder = PngEncoder::new(cursor);
        encoder
            .write_image(&buffer_source, width, height, color_type.into())
            .expect("Conversion should be possible");

        let current_image_base64 = BASE64_STANDARD.encode(buffer_target);
        rsx! {
            div {
                class: "page_reader",
                button {
                    onclick: move |_| current_img_idx.set(std::cmp::max(0i32, (idx as i32) - 1i32) as usize),
                    Icon {
                        icon: "\u{f060}".to_owned()
                    }
                }
                img {
                    src: "data:image/png;base64,{current_image_base64}",
                }
                button {
                    onclick: move |_| current_img_idx.set((idx + 1) % page_count),
                    Icon {
                        icon: "\u{f061}".to_owned()
                    }
                }
            }
        }
    }
}
