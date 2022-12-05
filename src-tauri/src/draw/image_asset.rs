#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug)]
pub struct ImageAsset {
    pub path: &'static str,
    pub point: Point<i32>
}

pub static DEFAULT_ASSETS: &'static [ImageAsset] = &[
    ImageAsset {
        path: "assets/img/ferris.png",
        point: Point {x: 1400, y: 950}
    },
    ImageAsset {
        path: "assets/img/crab(1).png",
        point: Point {x: 1500, y: 650}
    },
    ImageAsset {
        path: "assets/img/crab.png",
        point: Point {x: 1050, y: 1100}
    },
];

// impl ImageAsset {
//     pub fn default() -> [Self, ] {
//         vec![
//             ImageAsset {
//                 path: "assets/img/ferris.png".into(),
//                 point: Point {x: 1400, y: 950}
//             },
//             ImageAsset {
//                 path: "assets/img/crab(1).png".into(),
//                 point: Point {x: 1500, y: 650}
//             },
//             ImageAsset {
//                 path: "assets/img/crab.png".into(),
//                 point: Point {x: 1050, y: 1100}
//             },
//         ]
//     }
// }
