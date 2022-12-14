use font_kit::loader::FallbackFont;
use raqote::{DrawTarget, Source, StrokeStyle, PathBuilder, DrawOptions, SolidSource, Point};

use font_kit::family_name::FamilyName;
use font_kit::properties::{Properties, Weight};
use font_kit::source::SystemSource;

use crate::prelude::Result;

pub fn draw_rounded_rect(
    canvas: &mut DrawTarget,
    top_left: (f32, f32),
    bottom_right: (f32, f32),
    source: &Source,
    stroke_style: &StrokeStyle,
    radius: f32
) {
    let mut pb = PathBuilder::new();

    let x = top_left.0 ;
    let y = top_left.1 ;
    let w = (bottom_right.0 - top_left.0) ;
    let h = (bottom_right.1 - top_left.1) ;

    pb.move_to(x + radius , y);

    pb.line_to(x + w - radius , y);
    pb.quad_to(x + w, y, x + w, y + radius );

    pb.line_to(x + w, y + h - radius );
    pb.quad_to(x + w, y + h, x + w - radius , y + h);

    pb.line_to(x + radius , y + h);
    pb.quad_to(x, y + h, x, y + h - radius );

    pb.line_to(x, y + radius );
    pb.quad_to(x, y, x + radius , y);

    let path = pb.finish();

    canvas.fill(
        &path,
        // white
        &Source::Solid(SolidSource::from_unpremultiplied_argb(255u8, 255u8, 255u8, 255u8)),
        &DrawOptions::default()
    );
    canvas.stroke(&path, source, stroke_style, &DrawOptions::default());
}

pub fn draw_tail(
    dt: &mut DrawTarget,
    source: &Source,
    stroke_style: &StrokeStyle,
    point: (i32, i32),
) {
    let mut pb = PathBuilder::new();
    pb.move_to(20., 36.);
    pb.line_to(point.0 as f32, point.1 as f32);
    pb.line_to(200., 36.);

    let path = pb.finish();

    dt.fill(
        &path,
        // white
        &Source::Solid(SolidSource::from_unpremultiplied_argb(255u8, 255u8, 255u8, 255u8)),
        &DrawOptions::default()
    );

    dt.stroke(
        &path,
        &source,
        &stroke_style,
        &DrawOptions::new(),
    );
}

pub fn draw_text (
    dt: &mut DrawTarget,
    text: String,
    font_size: f32,
    line_height: f32,
    source: &Source,
) -> Result<()> {
    let font = SystemSource::new()
        .select_best_match(
            &[
                FamilyName::Title("Comic Sans MS".into()),
                FamilyName::Title("Arial".into()),
                FamilyName::Title("Verdana".into()),
            ],
            &Properties::new().weight(Weight::NORMAL)
        )?.load()?;

    for (i, line) in text.split('\n').enumerate() {
        dt.draw_text(
            &font, 
            font_size,
            line, 
            Point::new(20., font_size + 5. + i as u32 as f32 * line_height), 
            &source, 
            &DrawOptions::default()
        )
    }

    Ok(())
}
