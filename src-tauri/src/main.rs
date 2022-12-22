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
use std::sync::Mutex;
use tauri::{Manager, WindowEvent, InvokeError};

use rand::seq::SliceRandom;

#[tauri::command]
async fn save_image(write_path: String, message: String, color: String, state: tauri::State<'_, ImageState>, app_handle: tauri::AppHandle) -> Result<()> {
    let path = Path::new(&write_path);

    let asset = { // ez scope for early drop (easier :D)
        *state.asset.lock().unwrap()
    };

    if let Some(asset) = asset {
        let asset_path = app_handle.path_resolver().resolve_resource(&asset.path)
            .expect("Asset should exist");

        draw_image(
            path,
            &asset_path.as_path(),
            &asset.point,
            message,
            color
        ).await?;
        return Ok(());
    }
    Err(Error::Generic("Nothing to export".into()))
}

#[tauri::command]
async fn generate_image(message: String, color: String, state: tauri::State<'_, ImageState>, app_handle: tauri::AppHandle) -> Result<String> {
    let asset = DEFAULT_ASSETS.choose(&mut rand::thread_rng())
        .expect("Slice shouldn't be empty");
    *state.asset.lock().unwrap() = Some(asset);

    println!("Random asset: {:?}", asset);

    let path = app_handle.path_resolver().resolve_resource(asset.path)
        .expect("Asset should exist");

    draw_image(
        state.path.as_path(), path.as_path(), &asset.point,
        message, color
    ).await?;

    Ok(state.path.clone().into_os_string().into_string()
        .map_err(|_| {Error::Generic("Failed to convert path into string".into())}).expect("TEST2"))
}

pub struct ImageState {
    path: PathBuf,
    asset: Mutex<Option<&'static ImageAsset>>,
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
        asset: Mutex::new(None),
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
        .invoke_handler(tauri::generate_handler![save_image, generate_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

