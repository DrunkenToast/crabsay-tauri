#![allow(unused)] // TODO: remove later, beginning only

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod error;
mod prelude;
mod draw;

use crate::prelude::*;
use draw::generator::*;
use draw::image_asset::*;

use std::env;
use std::path::{Path, PathBuf};
use tauri::{Manager, WindowEvent, InvokeError};

use rand::seq::SliceRandom;

#[tauri::command]
fn greet(name: &str, state: tauri::State<ImageState>) -> String {
    "from Rust!".into()
}

#[tauri::command]
async fn generate_image(message: String, color: String, state: tauri::State<'_, ImageState>, app_handle: tauri::AppHandle) -> Result<String> {
    let asset = state.assets.choose(&mut rand::thread_rng())
        .expect("Slice shouldn't be empty");

    println!("Random asset: {:?}", asset);

    let path = app_handle.path_resolver().resolve_resource(&asset.path)
        .expect("Asset should exist");

    draw_image(state.path.as_path(), path.as_path(), &asset.point).await.expect("TEST");

    Ok(state.path.clone().into_os_string().into_string()
        .map_err(|_| {Error::Generic("Failed to convert path into string".into())}).expect("TEST2"))
}

pub struct ImageState {
    path: PathBuf,
    assets: Vec<ImageAsset>
}

impl Drop for ImageState {
    fn drop(&mut self) {
        println!("kaboom");
    }
}

fn main() {
    let state = ImageState {
        path: { 
            let mut buf = std::env::temp_dir();
            buf.push("crabsay");
            buf.set_extension("png");
            buf
        },
        assets: ImageAsset::default_assets()
    };

    tauri::Builder::default()
        .manage(state)
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested {..} => {
                println!("Exiting, cleaning up...");
                std::fs::remove_file(event.window().state::<ImageState>().path.as_path());
            }
            _ => {},
        })
        .invoke_handler(tauri::generate_handler![greet, generate_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

