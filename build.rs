extern crate image;
extern crate distance_field;

use std::fs::File;
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

    let ref mut fout = File::create(format!("resources/{}", name)).unwrap();

    image::ImageLuma8(outbuf).save(fout, image::PNG).unwrap();
}

fn main() {
    std::fs::create_dir_all("resources").unwrap();

    convert_image_to_dfield("rocket.png");
}
