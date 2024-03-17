#![allow(non_snake_case)]
use dioxus::prelude::*;

use image::buffer::ConvertBuffer;
use image::codecs::png::PngEncoder;
use image::DynamicImage;
use image::ImageFormat;
use image::load_from_memory;

use base64::prelude::*;

use zip::ZipArchive;

use std::fs::File;

use std::path::PathBuf;

use rfd::FileDialog;

use crate::components::icon::Icon;

use crate::models::project::Project;

fn import_zip_images(path: PathBuf, mut project: Signal<Project>) -> Result<(), String> {
    path.to_str()
        .ok_or("Path should be convertible into a string".to_owned())
        .and_then(|p| File::open(p).map_err(|err| err.to_string()))
        .and_then(|f| ZipArchive::new(f).map_err(|err| err.to_string()))
        .and_then(|mut ar| {
            (0..ar.len())
                .into_iter()
                .map(|i|
                    ar
                        .by_index(i)
                        .map_err(|err| err.to_string())
                        .and_then(|mut f| {
                            let mut writer: Vec<u8> = vec![];
                            std::io::copy(&mut f, &mut writer).expect("expect to succeed");
                            load_from_memory(&writer).map_err(|err| err.to_string())
                        })
                )
                .collect::<Result<Vec<DynamicImage>, _>>()
        })
        .map(|vec| {
            project.with_mut(move |p| {
                p.images = vec;
            })
        })
}

pub fn ImportButton() -> Element {
    let project = consume_context::<Signal<Project>>();

    rsx! {
        button {
            onclick: move |_| {
                FileDialog::new()
                .add_filter("zip", &["zip"])
                .set_directory("/")
                .pick_file()
                .map(|f| import_zip_images(f, project).map_err(|err| println!("{:?}", err)));
            },
            Icon {
                icon: "\u{f56f}".to_owned()
            }
        }
    }
}
