#![allow(non_snake_case)]
use dioxus::prelude::*;

use image::load_from_memory;


use zip::ZipArchive;

use std::fs::File;

use std::path::PathBuf;

use rfd::FileDialog;

use crate::components::icon::Icon;

use crate::models::page::Page;
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
                            load_from_memory(&writer)
                                .map_err(|err| err.to_string())
                                .map(|i| {
                                    let path = std::path::Path::new(f.name());
                                    Page::new(
                                        path
                                            .file_stem()
                                            .and_then(|s| s.to_str())
                                            .map(|s| s.to_string())
                                            .unwrap_or("Unknown image".to_string()),
                                        i
                                    )
                                })
                        })
                )
                .collect::<Result<Vec<Page>, _>>()
        })
        .map(|mut vec| {
            project.with_mut(move |p| {
                vec.sort_by_key(|i|i.name.clone());
                p.pages = vec;
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
