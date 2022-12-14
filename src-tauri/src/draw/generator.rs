use crate::draw::draw_utils::{draw_tail, draw_rounded_rect, draw_text};
use crate::draw::image_asset::*;
use crate::prelude::*;
use crate::draw::image_asset::Point;

use hex;
use image::imageops::FilterType;
use image::{ImageEncoder, GenericImage, ImageBuffer, open};
use imageproc::drawing::{
    draw_filled_rect_mut, draw_hollow_circle_mut, draw_hollow_rect_mut, draw_polygon_mut,
};
use imageproc::rect::Rect;
use raqote::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use image::Pixel;
use image::Pixels;

pub async fn draw_image(
    write_path: &Path,
    asset_path: &Path,
    asset_point: &Point<i32>,
    message: String,
    color: String,
) -> Result<()> {
    dbg!("Drawing now");
    let white = image::Rgba([255u8, 255u8, 255u8, 255u8]);

    let mut chars = color.chars();
    chars.next();
    let color = chars.as_str();

    let color = hex::decode(color).unwrap_or_else(|_| vec![0u8, 0u8, 0u8]);

    let mut color_source = raqote::SolidSource::from_unpremultiplied_argb(255u8, 0u8, 0u8, 0u8);

    if (color.len() >= 3) {
        color_source = raqote::SolidSource::from_unpremultiplied_argb(255u8, color[0], color[1], color[2]);
    }

    let color_source = &Source::Solid(color_source);

    let decoder = png::Decoder::new(File::open(&asset_path).unwrap());
    let (info, mut reader) = decoder.read_info()?;
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;

    println!("{:?}", info.color_type);

    let mut image: Vec<u32> = Vec::new();
    for i in buf.chunks(4) {
        image.push(0xff << 24 | ((i[0] as u32) << 16) | ((i[1] as u32) << 8) | (i[2] as u32))
    }

    let bitmap = Image {
        width: info.width as i32,
        height: info.height as i32,
        data: &image[..],
    };


    let mut dt = DrawTarget::new(bitmap.width, bitmap.height);
    dt.draw_image_at(0., 0., &bitmap, &DrawOptions::default());

    let stroke_style = StrokeStyle {
        width: 10.,
        ..StrokeStyle::default()
    };

    draw_tail(&mut dt, &color_source, &stroke_style, (asset_point.x, asset_point.y));

    let padding = 15 as f32;
    let font_size = 72 as f32;
    let line_height = font_size * 1.286; //approximation

    draw_rounded_rect(
        &mut dt, 
        (padding, padding), 
        (bitmap.width as f32 - padding, message.split('\n').count() as u32 as f32 * line_height), 
        &color_source, 
        &stroke_style, 
        20.
    );

    draw_text(
        &mut dt,
        message,
        font_size,
        line_height,
        &color_source
    )?;

    dbg!("Saving to:", &write_path);
    dt.write_png(&write_path).map_err(|_| Error::PngEncoding)?;

    Ok(())
}
