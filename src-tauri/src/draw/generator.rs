use crate::prelude::*;
use crate::draw::image_asset::*;

use image::ImageEncoder;
use imageproc::drawing::{
    draw_filled_rect_mut, draw_hollow_circle_mut,
};
use imageproc::rect::Rect;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use image::codecs::png::{PngEncoder, CompressionType, FilterType};

pub async fn draw_image(write_path: &Path, asset_path: &Path, asset_point: &Point) -> Result<()> {
    let white = image::Rgba([255u8, 255u8, 255u8, 255u8]);

    let mut image = image::open(asset_path).expect("Test open");

    draw_filled_rect_mut(&mut image, Rect::at(130, 10).of_size(20, 20), white);
    draw_filled_rect_mut(&mut image, Rect::at(300, 10).of_size(20, 20), white);
    draw_filled_rect_mut(&mut image, Rect::at(180, -10).of_size(30, 20), white);

    draw_hollow_circle_mut(&mut image, (100, 100), 15, white);
    draw_hollow_circle_mut(&mut image, (400, 400), 20, white);
    draw_hollow_circle_mut(&mut image, (100, 190), 20, white);

    println!("Start writing");

    image.save(&write_path).unwrap();

    println!("Finished writing");

    Ok(())
        // .map_err(|e|{Error::IoError("Failed to save image".into())})?)
}
