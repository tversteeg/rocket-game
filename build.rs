extern crate distance_field;
extern crate image;

use std::path::Path;

use distance_field::DistanceFieldExt;

fn convert_image_to_dfield(name: &str) {
    if Path::new(&format!("resources/{}", name)).exists() {
        return;
    }

    let img = image::open(format!("assets/{}", name)).unwrap();

    let outbuf = img.grayscale().distance_field(distance_field::Options {
        size: (128, 128),
        max_distance: 256,
        ..Default::default()
    });

    image::ImageLuma8(outbuf)
        .save(format!("resources/{}", name))
        .unwrap();
}

fn main() {
    std::fs::create_dir_all("resources").unwrap();

    convert_image_to_dfield("rocket.png");
}
