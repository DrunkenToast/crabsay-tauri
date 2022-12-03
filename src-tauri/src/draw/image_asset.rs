#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct ImageAsset {
    pub path: String,
    pub point: Point
}

impl ImageAsset {
    pub fn default_assets() -> Vec<Self> {
        vec![
            ImageAsset {
                path: "assets/img/chillin.jpg".into(),
                point: Point {x: 2000, y: 1200}
            },
            ImageAsset {
                path: "assets/img/blurrycow.jpg".into(),
                point: Point {x: 700, y: 1800}
            },
            ImageAsset {
                path: "assets/img/sup.jpg".into(),
                point: Point {x: 700, y: 1000}
            },
        ]
    }
}
